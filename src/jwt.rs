use std::collections::HashMap;

use anyhow::{Error, Result};
use jsonwebtoken::{Algorithm, decode, decode_header, DecodingKey, Validation};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub(crate) user_id: String,
    pub(crate) aud: String,
}

pub async fn claims(jwt: &str) -> Result<Claims> {
    let header = decode_header(jwt)?;
    let kid = header.kid.ok_or_else(|| Error::msg("missing kid header"))?;
    let key = get_key(&kid).await.and_then(|key| get_public(&key))?;

    decode::<Claims>(jwt, &DecodingKey::from_rsa_pem(key.as_ref())?, &Validation::new(Algorithm::RS256)).map(|data| data.claims).map_err(Error::from)
}

async fn get_key(id: &str) -> Result<String> {
    reqwest::get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")
        .await
        .map_err(Error::from)?.json::<HashMap<String, String>>()
        .await
        .map_err(Error::from)
        .and_then(|m| m.get(id).ok_or_else(|| Error::msg("id not found")).map(|k| k.to_owned()))
}

fn get_public(pem: &str) -> Result<Vec<u8>> {
    let x509 = openssl::x509::X509::from_pem(pem.as_ref())?;
    let pkey = x509.public_key()?.public_key_to_pem()?;

    Ok(pkey)
}