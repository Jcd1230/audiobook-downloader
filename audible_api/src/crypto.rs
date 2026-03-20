use base64::{engine::general_purpose::URL_SAFE_NO_PAD, engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::RsaPrivateKey;
use sha2::{Sha256, Digest};
use rand::RngCore;
use anyhow::{Context, Result};
use reqwest::Request;

/// Generates a PKCE code verifier and corresponding S256 code challenge.
pub fn generate_pkce_pair() -> (String, String) {
    let mut verifier_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut verifier_bytes);
    
    let verifier = URL_SAFE_NO_PAD.encode(verifier_bytes);
    
    let mut hasher = Sha256::new();
    hasher.update(&verifier_bytes); // audible-cli uses verifier_bytes directly for the hash
    let challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());
    
    (verifier, challenge)
}

/// Generates the standard Audible iOS client ID from a device serial string.
pub fn build_client_id(serial: &str) -> String {
    let raw = format!("{}#A2CZJZGLK2JJVM", serial);
    hex::encode(raw) // audible-cli uses hex encoding of the byte string
}

/// Cryptographically signs a request intended for DRM endpoints using the RSA device key.
pub fn sign_request(
    request: &mut Request,
    adp_token: &str,
    device_private_key: &str,
) -> Result<()> {
    let method = request.method().as_str().to_uppercase();
    
    let mut url = request.url().path().to_string();
    if let Some(query) = request.url().query() {
        url = format!("{}?{}", url, query);
    }
    
    // Python datetime.now(timezone.utc).isoformat("T") + "Z" produces:
    // e.g., 2026-03-20T23:10:12.476690+00:00Z
    let date = Utc::now().format("%Y-%m-%dT%H:%M:%S.%f+00:00Z").to_string();
    
    // Read the body if it's there
    let body_str = if let Some(body) = request.body() {
        if let Some(b) = body.as_bytes() {
            String::from_utf8_lossy(b).into_owned()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Construct the payload to sign
    let data_string = format!("{}\n{}\n{}\n{}\n{}", method, url, date, body_str, adp_token);
    
    // Parse the RSA Private Key
    let rsa_key = RsaPrivateKey::from_pkcs1_pem(device_private_key)
        .context("Failed to parse device private key PEM")?;
        
    let mut rng = rand::thread_rng();
    
    let mut hasher = Sha256::new();
    hasher.update(data_string.as_bytes());
    let hashed = hasher.finalize();
    
    let padding = rsa::pkcs1v15::Pkcs1v15Sign::new::<Sha256>();
    let signature_bytes = rsa_key.sign(padding, &hashed)
        .context("Failed to sign request string")?;

    let signature_b64 = STANDARD.encode(signature_bytes);
    let final_signature = format!("{}:{}", signature_b64, date);

    let headers = request.headers_mut();
    headers.insert("x-adp-token", adp_token.parse()?);
    headers.insert("x-adp-alg", "SHA256withRSA:1.0".parse()?);
    headers.insert("x-adp-signature", final_signature.parse()?);

    Ok(())
}
