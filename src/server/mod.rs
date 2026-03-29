#[cfg(feature = "ssr")]
pub mod auth;
#[cfg(feature = "ssr")]
pub mod config;
pub mod functions;
#[cfg(feature = "ssr")]
pub mod storage;

#[cfg(feature = "ssr")]
use axum::{
    extract::{Extension, Multipart, Path},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
use crate::models::MenuData;
#[cfg(feature = "ssr")]
use config::Config;
#[cfg(feature = "ssr")]
use storage::Storage;

/// Handler Axum dédié à l'upload de photo (multipart).
/// Accessible sur POST /api/admin/photo/:id
#[cfg(feature = "ssr")]
pub async fn upload_photo_handler(
    Extension(storage): Extension<Arc<Storage>>,
    Extension(config):  Extension<Arc<Config>>,
    Path(item_id):      Path<String>,
    headers:            HeaderMap,
    mut multipart:      Multipart,
) -> impl IntoResponse {
    if !auth::verify_session(&headers, &config.jwt_secret) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error":"Non autorisé"}))).into_response();
    }

    let mut menu_data: MenuData = storage
        .get_json_or_default("data/menu.json")
        .await
        .unwrap_or_default();

    let item = match menu_data.items.iter_mut().find(|i| i.id == item_id) {
        Some(i) => i,
        None => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error":"Plat introuvable"}))).into_response(),
    };

    while let Ok(Some(field)) = multipart.next_field().await {
        let content_type = field.content_type().unwrap_or("image/jpeg").to_string();
        let ext = match content_type.as_str() {
            "image/png"  => "png",
            "image/gif"  => "gif",
            "image/webp" => "webp",
            _            => "jpg",
        };

        let bytes = match field.bytes().await {
            Ok(b) if !b.is_empty() => b.to_vec(),
            _ => continue,
        };

        let key = format!("photos/{}.{}", Uuid::new_v4(), ext);
        match storage.put_photo(&key, bytes, content_type).await {
            Ok(url) => {
                item.photo_url = Some(url.clone());
                let _ = storage.put_json("data/menu.json", &menu_data).await;
                return (StatusCode::OK, Json(serde_json::json!({"photo_url": url}))).into_response();
            }
            Err(e) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response();
            }
        }
    }

    (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error":"Aucun fichier reçu"}))).into_response()
}
