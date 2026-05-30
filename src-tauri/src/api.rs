use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize)]
pub struct RegisterPayload {
    pub username: String,
    pub passphrase_hash: String,
    pub pubkey: String,
    pub encrypted_privkey: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub encrypted_privkey: String,
    #[allow(dead_code)]
    pub pubkey: String,
}

#[derive(Deserialize)]
pub struct UserResponse {
    pub pubkey: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bottle {
    pub id: String,
    pub from: String,
    pub to: String,
    pub body: String,
    pub sender_pubkey: Option<String>,
    pub encrypted: bool,
    pub timestamp: u64,
}

#[derive(Deserialize)]
pub struct BottleMeta {
    pub id: String,
    pub from: String,
    pub encrypted: bool,
    pub timestamp: u64,
}

pub fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new())
}

pub fn passphrase_hash(username: &str, passphrase: &str) -> String {
    let mut h = Sha256::new();
    h.update(username.as_bytes());
    h.update(b":");
    h.update(passphrase.as_bytes());
    hex::encode(h.finalize())
}

// unused but kept for clarity — auth_header is inlined in caller functions
pub async fn register(
    client: &Client,
    worker_url: &str,
    payload: RegisterPayload,
) -> Result<(), String> {
    let res = client
        .post(format!("{worker_url}/register"))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err(res.text().await.unwrap_or_default())
    }
}

pub async fn login(
    client: &Client,
    worker_url: &str,
    username: &str,
    passphrase: &str,
) -> Result<LoginResponse, String> {
    let hash = passphrase_hash(username, passphrase);
    let res = client
        .post(format!("{worker_url}/login"))
        .json(&serde_json::json!({ "username": username, "passphrase_hash": hash }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if res.status().is_success() {
        res.json().await.map_err(|e| e.to_string())
    } else {
        Err(res.text().await.unwrap_or_default())
    }
}

pub async fn get_user_pubkey(
    client: &Client,
    worker_url: &str,
    username: &str,
) -> Result<String, String> {
    let res = client
        .get(format!("{worker_url}/user/{username}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if res.status().is_success() {
        let u: UserResponse = res.json().await.map_err(|e| e.to_string())?;
        Ok(u.pubkey)
    } else {
        Err(res.text().await.unwrap_or_default())
    }
}

pub async fn throw_bottle(
    client: &Client,
    worker_url: &str,
    username: &str,
    passphrase_hash: &str,
    bottle: &Bottle,
) -> Result<(), String> {
    let creds = URL_SAFE_NO_PAD.encode(format!("{username}:{passphrase_hash}"));
    let res = client
        .post(format!("{worker_url}/throw"))
        .header("Authorization", format!("Basic {creds}"))
        .json(bottle)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err(res.text().await.unwrap_or_default())
    }
}

pub async fn fetch_bottles(
    client: &Client,
    worker_url: &str,
    username: &str,
    passphrase_hash: &str,
) -> Result<Vec<BottleMeta>, String> {
    let creds = URL_SAFE_NO_PAD.encode(format!("{username}:{passphrase_hash}"));
    let res = client
        .get(format!("{worker_url}/bottles/{username}"))
        .header("Authorization", format!("Basic {creds}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if res.status().is_success() {
        res.json().await.map_err(|e| e.to_string())
    } else {
        Err(res.text().await.unwrap_or_default())
    }
}

pub async fn get_bottle(
    client: &Client,
    worker_url: &str,
    username: &str,
    passphrase_hash: &str,
    id: &str,
) -> Result<Bottle, String> {
    let creds = URL_SAFE_NO_PAD.encode(format!("{username}:{passphrase_hash}"));
    let res = client
        .get(format!("{worker_url}/bottle/{id}"))
        .header("Authorization", format!("Basic {creds}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if res.status().is_success() {
        res.json().await.map_err(|e| e.to_string())
    } else {
        Err(res.text().await.unwrap_or_default())
    }
}

pub async fn delete_bottle(
    client: &Client,
    worker_url: &str,
    username: &str,
    passphrase_hash: &str,
    id: &str,
) -> Result<(), String> {
    let creds = URL_SAFE_NO_PAD.encode(format!("{username}:{passphrase_hash}"));
    let res = client
        .delete(format!("{worker_url}/bottle/{id}"))
        .header("Authorization", format!("Basic {creds}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err(res.text().await.unwrap_or_default())
    }
}
