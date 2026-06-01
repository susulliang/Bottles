use std::{net::TcpStream, time::Duration};

use base64::{engine::general_purpose::STANDARD, Engine};
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
    let mut builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .tcp_keepalive(Duration::from_secs(30));

    if let Some(proxy) = proxy_url().and_then(|proxy_url| reqwest::Proxy::all(&proxy_url).ok()) {
        builder = builder.proxy(proxy);
    }

    builder.build().unwrap_or_else(|_| reqwest::Client::new())
}

fn proxy_url() -> Option<String> {
    for key in [
        "HTTPS_PROXY",
        "https_proxy",
        "ALL_PROXY",
        "all_proxy",
        "HTTP_PROXY",
        "http_proxy",
    ] {
        if let Ok(proxy_url) = std::env::var(key) {
            if !proxy_url.is_empty() {
                return Some(proxy_url);
            }
        }
    }

    if TcpStream::connect_timeout(
        &"127.0.0.1:7897".parse().expect("valid proxy address"),
        Duration::from_millis(200),
    )
    .is_ok()
    {
        return Some("http://127.0.0.1:7897".into());
    }

    None
}

fn request_error(url: &str, err: reqwest::Error) -> String {
    let proxy = proxy_url().unwrap_or_else(|| "未检测到代理".into());
    format!("请求 {url} 失败：{err}。当前代理：{proxy}")
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
    let url = format!("{worker_url}/register");
    let res = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| request_error(&url, e))?;
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
    let url = format!("{worker_url}/login");
    let res = client
        .post(&url)
        .json(&serde_json::json!({ "username": username, "passphrase_hash": hash }))
        .send()
        .await
        .map_err(|e| request_error(&url, e))?;
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
    let url = format!("{worker_url}/user/{username}");
    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| request_error(&url, e))?;
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
    let creds = STANDARD.encode(format!("{username}:{passphrase_hash}"));
    let url = format!("{worker_url}/throw");
    let res = client
        .post(&url)
        .header("Authorization", format!("Basic {creds}"))
        .json(bottle)
        .send()
        .await
        .map_err(|e| request_error(&url, e))?;
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
    let creds = STANDARD.encode(format!("{username}:{passphrase_hash}"));
    let url = format!("{worker_url}/bottles/{username}");
    let res = client
        .get(&url)
        .header("Authorization", format!("Basic {creds}"))
        .send()
        .await
        .map_err(|e| request_error(&url, e))?;
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
    let creds = STANDARD.encode(format!("{username}:{passphrase_hash}"));
    let url = format!("{worker_url}/bottle/{id}");
    let res = client
        .get(&url)
        .header("Authorization", format!("Basic {creds}"))
        .send()
        .await
        .map_err(|e| request_error(&url, e))?;
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
    let creds = STANDARD.encode(format!("{username}:{passphrase_hash}"));
    let url = format!("{worker_url}/bottle/{id}");
    let res = client
        .delete(&url)
        .header("Authorization", format!("Basic {creds}"))
        .send()
        .await
        .map_err(|e| request_error(&url, e))?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err(res.text().await.unwrap_or_default())
    }
}

pub async fn fetch_sent_bottles(
    client: &Client,
    worker_url: &str,
    username: &str,
    passphrase_hash: &str,
) -> Result<Vec<BottleMeta>, String> {
    let creds = STANDARD.encode(format!("{username}:{passphrase_hash}"));
    let url = format!("{worker_url}/sent/{username}");
    let res = client
        .get(&url)
        .header("Authorization", format!("Basic {creds}"))
        .send()
        .await
        .map_err(|e| request_error(&url, e))?;
    if res.status().is_success() {
        res.json().await.map_err(|e| e.to_string())
    } else {
        Err(res.text().await.unwrap_or_default())
    }
}
