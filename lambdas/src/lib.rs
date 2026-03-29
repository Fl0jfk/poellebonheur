use anyhow::Result;
use aws_sdk_s3::{primitives::ByteStream, Client};
use lambda_http::{Body, Response};
use serde::{Deserialize, Serialize};

// ── Modèles partagés ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MarketInfo {
    pub date:   Option<String>,
    pub place:  Option<String>,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum QuoteStatus {
    #[default]
    Pending,
    Viewed,
    Replied,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuoteRequest {
    pub id:               String,
    pub last_name:        String,
    pub first_name:       String,
    pub phone:            String,
    pub email:            String,
    pub event_date:       String,
    pub event_place:      String,
    pub number_of_people: u32,
    pub starters:         Vec<String>,
    pub main_dish:        String,
    pub desserts:         Vec<String>,
    pub message:          Option<String>,
    pub created_at:       String,
    pub status:           QuoteStatus,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QuotesData {
    pub quotes: Vec<QuoteRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuItem {
    pub id:          String,
    pub name:        String,
    pub description: String,
    pub photo_url:   Option<String>,
    pub category:    String,
    pub price_info:  Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MenuData {
    pub items: Vec<MenuItem>,
}

// ── S3 helpers ────────────────────────────────────────────────────────────────

pub fn bucket() -> String {
    std::env::var("S3_BUCKET_NAME").unwrap_or_else(|_| "poellebonheur".to_string())
}

pub async fn s3_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

pub async fn s3_get<T: serde::de::DeserializeOwned + Default>(key: &str) -> T {
    let client = s3_client().await;
    let resp = client
        .get_object()
        .bucket(bucket())
        .key(key)
        .send()
        .await;
    match resp {
        Ok(out) => {
            let data = out.body.collect().await
                .map(|b| b.into_bytes())
                .unwrap_or_default();
            serde_json::from_slice(&data).unwrap_or_default()
        }
        Err(_) => T::default(),
    }
}

pub async fn s3_put<T: serde::Serialize>(key: &str, data: &T) -> Result<()> {
    let client = s3_client().await;
    let json = serde_json::to_vec(data)?;
    client
        .put_object()
        .bucket(bucket())
        .key(key)
        .body(ByteStream::from(json))
        .content_type("application/json")
        .send()
        .await?;
    Ok(())
}

// ── Réponses HTTP avec CORS ───────────────────────────────────────────────────

pub fn ok(body: impl Into<String>) -> Response<Body> {
    json_response(200, body.into())
}

pub fn err(status: u16, msg: impl std::fmt::Display) -> Response<Body> {
    json_response(status, format!(r#"{{"error":"{msg}"}}"#))
}

pub fn preflight() -> Response<Body> {
    json_response(200, String::new())
}

fn json_response(status: u16, body: String) -> Response<Body> {
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, x-admin-key")
        .body(Body::Text(body))
        .unwrap()
}

// ── Vérification clé admin ────────────────────────────────────────────────────

pub fn check_admin_key(req: &lambda_http::Request) -> bool {
    let expected = std::env::var("ADMIN_API_KEY").unwrap_or_default();
    if expected.is_empty() {
        return true; // pas de clé configurée → dev mode
    }
    req.headers()
        .get("x-admin-key")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == expected)
        .unwrap_or(false)
}
