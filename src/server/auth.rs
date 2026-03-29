use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims { sub: "admin".into(), exp };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
        .map(|d| d.claims)
}

/// Vérifie le cookie de session depuis les headers HTTP.
pub fn verify_session(headers: &HeaderMap, secret: &str) -> bool {
    let cookie_val = headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    extract_session_token(cookie_val)
        .map(|tok| verify_jwt(&tok, secret).is_ok())
        .unwrap_or(false)
}

/// Extrait le token du cookie `admin_session=<token>`.
pub fn extract_session_token(cookie_str: &str) -> Option<String> {
    for part in cookie_str.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("admin_session=") {
            if !val.is_empty() {
                return Some(val.to_string());
            }
        }
    }
    None
}
