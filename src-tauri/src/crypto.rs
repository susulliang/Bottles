use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::{Digest, Sha256};
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

pub fn generate_keypair() -> ([u8; 32], [u8; 32]) {
    let secret = StaticSecret::random_from_rng(rand::thread_rng());
    let public = PublicKey::from(&secret);
    (*secret.as_bytes(), *public.as_bytes())
}

fn passphrase_key(passphrase: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(passphrase.as_bytes());
    h.finalize().into()
}

fn passphrase_nonce(passphrase: &str) -> [u8; 12] {
    let mut h = Sha256::new();
    h.update(passphrase.as_bytes());
    h.update(b"nonce");
    let d = h.finalize();
    d[..12].try_into().unwrap()
}

pub fn encrypt_privkey(privkey: &[u8; 32], passphrase: &str) -> String {
    let key = passphrase_key(passphrase);
    let nonce_bytes = passphrase_nonce(passphrase);
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ct = cipher.encrypt(nonce, privkey.as_ref()).unwrap();
    URL_SAFE_NO_PAD.encode(ct)
}

pub fn decrypt_privkey(b64: &str, passphrase: &str) -> Result<[u8; 32], String> {
    let ct = URL_SAFE_NO_PAD.decode(b64).map_err(|e| e.to_string())?;
    let key = passphrase_key(passphrase);
    let nonce_bytes = passphrase_nonce(passphrase);
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let pt = cipher
        .decrypt(nonce, ct.as_ref())
        .map_err(|_| "decryption failed — wrong passphrase".to_string())?;
    pt.try_into().map_err(|_| "invalid key length".to_string())
}

pub fn encrypt_message(
    plaintext: &str,
    recipient_pubkey_b64: &str,
    our_privkey: &[u8; 32],
) -> Result<String, String> {
    let recipient_bytes: [u8; 32] = URL_SAFE_NO_PAD
        .decode(recipient_pubkey_b64)
        .map_err(|e| e.to_string())?
        .try_into()
        .map_err(|_| "bad pubkey length".to_string())?;

    let ephemeral_secret = EphemeralSecret::random_from_rng(rand::thread_rng());
    let ephemeral_pub = PublicKey::from(&ephemeral_secret);

    // ECDH with recipient pubkey
    let recipient_pub = PublicKey::from(recipient_bytes);
    let shared = ephemeral_secret.diffie_hellman(&recipient_pub);

    // Also mix in our static privkey for sender authentication
    let static_secret = StaticSecret::from(*our_privkey);
    let static_shared = static_secret.diffie_hellman(&recipient_pub);

    // Derive AES key via HKDF
    let mut ikm = [0u8; 64];
    ikm[..32].copy_from_slice(shared.as_bytes());
    ikm[32..].copy_from_slice(static_shared.as_bytes());
    let hk = Hkdf::<Sha256>::new(None, &ikm);
    let mut aes_key = [0u8; 32];
    hk.expand(b"bottles-msg", &mut aes_key).unwrap();

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(&aes_key).unwrap();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ct = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| e.to_string())?;

    // wire: ephemeral_pub(32) + nonce(12) + ciphertext
    let mut wire = Vec::with_capacity(32 + 12 + ct.len());
    wire.extend_from_slice(ephemeral_pub.as_bytes());
    wire.extend_from_slice(&nonce_bytes);
    wire.extend_from_slice(&ct);

    Ok(URL_SAFE_NO_PAD.encode(wire))
}

pub fn decrypt_message(
    b64: &str,
    sender_pubkey_b64: &str,
    our_privkey: &[u8; 32],
) -> Result<String, String> {
    let wire = URL_SAFE_NO_PAD.decode(b64).map_err(|e| e.to_string())?;
    if wire.len() < 44 {
        return Err("message too short".to_string());
    }
    let ephemeral_pub = PublicKey::from(<[u8; 32]>::try_from(&wire[..32]).unwrap());
    let nonce_bytes: [u8; 12] = wire[32..44].try_into().unwrap();
    let ct = &wire[44..];

    let sender_bytes: [u8; 32] = URL_SAFE_NO_PAD
        .decode(sender_pubkey_b64)
        .map_err(|e| e.to_string())?
        .try_into()
        .map_err(|_| "bad pubkey length".to_string())?;
    let sender_pub = PublicKey::from(sender_bytes);

    let our_secret = StaticSecret::from(*our_privkey);
    let shared = our_secret.diffie_hellman(&ephemeral_pub);
    let static_shared = our_secret.diffie_hellman(&sender_pub);

    let mut ikm = [0u8; 64];
    ikm[..32].copy_from_slice(shared.as_bytes());
    ikm[32..].copy_from_slice(static_shared.as_bytes());
    let hk = Hkdf::<Sha256>::new(None, &ikm);
    let mut aes_key = [0u8; 32];
    hk.expand(b"bottles-msg", &mut aes_key).unwrap();

    let cipher = Aes256Gcm::new_from_slice(&aes_key).unwrap();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let pt = cipher
        .decrypt(nonce, ct)
        .map_err(|_| "decryption failed".to_string())?;

    String::from_utf8(pt).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn privkey_roundtrip() {
        let (priv1, _) = generate_keypair();
        let enc = encrypt_privkey(&priv1, "hunter2");
        let dec = decrypt_privkey(&enc, "hunter2").unwrap();
        assert_eq!(priv1, dec);
    }

    #[test]
    fn privkey_wrong_passphrase() {
        let (priv1, _) = generate_keypair();
        let enc = encrypt_privkey(&priv1, "hunter2");
        assert!(decrypt_privkey(&enc, "wrong").is_err());
    }

    #[test]
    fn message_roundtrip() {
        let (alice_priv, alice_pub) = generate_keypair();
        let (bob_priv, bob_pub) = generate_keypair();
        let alice_pub_b64 = URL_SAFE_NO_PAD.encode(alice_pub);
        let bob_pub_b64 = URL_SAFE_NO_PAD.encode(bob_pub);

        let ct = encrypt_message("hello bob", &bob_pub_b64, &alice_priv).unwrap();
        let pt = decrypt_message(&ct, &alice_pub_b64, &bob_priv).unwrap();
        assert_eq!(pt, "hello bob");
    }
}
