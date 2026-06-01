use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use crate::{
    api::{self, client},
    crypto,
    state::AppState,
};

#[derive(Serialize, Deserialize)]
pub struct BottleMetaOut {
    pub id: String,
    pub from: String,
    pub encrypted: bool,
    pub timestamp: u64,
}

#[derive(Serialize)]
pub struct BottleContentOut {
    pub id: String,
    pub from: String,
    pub body: String,
    pub timestamp: u64,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[tauri::command]
pub async fn register(
    username: String,
    passphrase: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let (privkey, pubkey) = crypto::generate_keypair();
    let encrypted_privkey = crypto::encrypt_privkey(&privkey, &passphrase);
    let pubkey_b64 = URL_SAFE_NO_PAD.encode(pubkey);
    let ph = api::passphrase_hash(&username, &passphrase);

    api::register(
        &client(),
        &state.worker_url,
        api::RegisterPayload {
            username: username.clone(),
            passphrase_hash: ph.clone(),
            pubkey: pubkey_b64,
            encrypted_privkey,
        },
    )
    .await?;

    *state.session.lock().unwrap() = Some(crate::state::Session {
        username,
        privkey,
        passphrase_hash: ph,
    });
    Ok(())
}

#[tauri::command]
pub async fn login(
    username: String,
    passphrase: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let resp = api::login(&client(), &state.worker_url, &username, &passphrase).await?;
    let privkey = crypto::decrypt_privkey(&resp.encrypted_privkey, &passphrase)?;
    let ph = api::passphrase_hash(&username, &passphrase);

    *state.session.lock().unwrap() = Some(crate::state::Session {
        username,
        privkey,
        passphrase_hash: ph,
    });
    Ok(())
}

#[tauri::command]
pub async fn login_or_register(
    username: String,
    passphrase: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    match login(username.clone(), passphrase.clone(), state.clone()).await {
        Ok(()) => Ok(false),
        Err(login_error) => match register(username, passphrase, state).await {
            Ok(()) => Ok(true),
            Err(register_error) => {
                if register_error.contains("username taken") {
                    Err(login_error)
                } else {
                    Err(register_error)
                }
            }
        },
    }
}

#[tauri::command]
pub async fn minimize_app(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "main window not found".to_string())?;
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn exit_app(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.close();
    }
    app.exit(0);
}

#[tauri::command]
pub fn logout(state: State<'_, AppState>) {
    *state.session.lock().unwrap() = None;
    state.known_pubkeys.lock().unwrap().clear();
}

#[tauri::command]
pub async fn throw_bottle(
    to: String,
    body: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let (username, privkey, ph) = {
        let s = state.session.lock().unwrap();
        let s = s.as_ref().ok_or("not logged in")?;
        (s.username.clone(), s.privkey, s.passphrase_hash.clone())
    };

    let recipient_pubkey_b64 = api::get_user_pubkey(&client(), &state.worker_url, &to)
        .await
        .map_err(|_| format!("Recipient '{to}' was not found. Please check the username and make sure they have registered before throwing a bottle."))?;

    let recipient_bytes: [u8; 32] = URL_SAFE_NO_PAD
        .decode(&recipient_pubkey_b64)
        .map_err(|e| e.to_string())?
        .try_into()
        .map_err(|_| "bad pubkey")?;

    let is_new = {
        let mut known = state.known_pubkeys.lock().unwrap();
        let is_new = !known.contains_key(&to);
        known.insert(to.clone(), recipient_bytes);
        is_new
    };

    let (body_out, encrypted, sender_pubkey) = if is_new {
        let our_pub = x25519_dalek::PublicKey::from(&x25519_dalek::StaticSecret::from(privkey));
        let our_pub_b64 = URL_SAFE_NO_PAD.encode(our_pub.as_bytes());
        (body, false, Some(our_pub_b64))
    } else {
        let ct = crypto::encrypt_message(&body, &recipient_pubkey_b64, &privkey)?;
        (ct, true, None)
    };

    let bottle = api::Bottle {
        id: Uuid::new_v4().to_string(),
        from: username.clone(),
        to,
        body: body_out,
        sender_pubkey,
        encrypted,
        timestamp: now(),
    };

    api::throw_bottle(&client(), &state.worker_url, &username, &ph, &bottle).await
}

#[tauri::command]
pub async fn fetch_bottles(state: State<'_, AppState>) -> Result<Vec<BottleMetaOut>, String> {
    let (username, ph) = {
        let s = state.session.lock().unwrap();
        let s = s.as_ref().ok_or("not logged in")?;
        (s.username.clone(), s.passphrase_hash.clone())
    };
    let metas = api::fetch_bottles(&client(), &state.worker_url, &username, &ph).await?;
    Ok(metas
        .into_iter()
        .map(|m| BottleMetaOut {
            id: m.id,
            from: m.from,
            encrypted: m.encrypted,
            timestamp: m.timestamp,
        })
        .collect())
}

#[tauri::command]
pub async fn open_bottle(
    id: String,
    state: State<'_, AppState>,
) -> Result<BottleContentOut, String> {
    let (username, privkey, ph) = {
        let s = state.session.lock().unwrap();
        let s = s.as_ref().ok_or("not logged in")?;
        (s.username.clone(), s.privkey, s.passphrase_hash.clone())
    };

    let bottle = api::get_bottle(&client(), &state.worker_url, &username, &ph, &id).await?;

    if let Some(ref spk) = bottle.sender_pubkey {
        let bytes: [u8; 32] = URL_SAFE_NO_PAD
            .decode(spk)
            .map_err(|e| e.to_string())?
            .try_into()
            .map_err(|_| "bad pubkey length")?;
        state
            .known_pubkeys
            .lock()
            .unwrap()
            .insert(bottle.from.clone(), bytes);
    }

    let body = if bottle.encrypted {
        let sender_pub_b64 = state
            .known_pubkeys
            .lock()
            .unwrap()
            .get(&bottle.from)
            .map(|b| URL_SAFE_NO_PAD.encode(b))
            .ok_or("no pubkey cached for sender")?;
        crypto::decrypt_message(&bottle.body, &sender_pub_b64, &privkey)?
    } else {
        bottle.body
    };

    Ok(BottleContentOut {
        id: bottle.id,
        from: bottle.from,
        body,
        timestamp: bottle.timestamp,
    })
}

#[tauri::command]
pub async fn delete_bottle(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let (username, ph) = {
        let s = state.session.lock().unwrap();
        let s = s.as_ref().ok_or("not logged in")?;
        (s.username.clone(), s.passphrase_hash.clone())
    };
    api::delete_bottle(&client(), &state.worker_url, &username, &ph, &id).await
}

#[tauri::command]
pub async fn fetch_sent_bottles(state: State<'_, AppState>) -> Result<Vec<BottleMetaOut>, String> {
    let (username, ph) = {
        let s = state.session.lock().unwrap();
        let s = s.as_ref().ok_or("not logged in")?;
        (s.username.clone(), s.passphrase_hash.clone())
    };
    let metas = api::fetch_sent_bottles(&client(), &state.worker_url, &username, &ph).await?;
    Ok(metas
        .into_iter()
        .map(|m| BottleMetaOut {
            id: m.id,
            from: m.from,
            encrypted: m.encrypted,
            timestamp: m.timestamp,
        })
        .collect())
}
