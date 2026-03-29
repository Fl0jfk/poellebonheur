use leptos::*;
#[cfg(feature = "ssr")]
use std::sync::Arc;

use crate::models::*;

// ── Helpers partagés (server only) ───────────────────────────────────────────

#[cfg(feature = "ssr")]
fn get_storage() -> Result<Arc<super::storage::Storage>, ServerFnError> {
    use_context::<Arc<super::storage::Storage>>()
        .ok_or_else(|| ServerFnError::new("Storage non disponible"))
}

#[cfg(feature = "ssr")]
fn get_config() -> Result<Arc<super::config::Config>, ServerFnError> {
    use_context::<Arc<super::config::Config>>()
        .ok_or_else(|| ServerFnError::new("Config non disponible"))
}

#[cfg(feature = "ssr")]
async fn require_admin() -> Result<(), ServerFnError> {
    use axum::http::HeaderMap;
    use super::auth::{extract_session_token, verify_jwt};

    let headers: HeaderMap = leptos_axum::extract().await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let config  = get_config()?;
    let cookie  = headers.get("cookie").and_then(|v| v.to_str().ok()).unwrap_or("");
    let token   = extract_session_token(cookie).ok_or_else(|| ServerFnError::new("Non autorisé"))?;
    verify_jwt(&token, &config.jwt_secret).map_err(|_| ServerFnError::new("Session expirée"))?;
    Ok(())
}

// ── AUTH ─────────────────────────────────────────────────────────────────────

#[server(AdminLogin, "/api")]
pub async fn admin_login(password: String) -> Result<(), ServerFnError> {
    use axum::http::{header, HeaderValue};
    use leptos_axum::ResponseOptions;
    use super::auth::create_jwt;

    let config = get_config()?;

    let valid = bcrypt::verify(&password, &config.admin_password_hash)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    if !valid {
        return Err(ServerFnError::new("Mot de passe incorrect"));
    }

    let token = create_jwt(&config.jwt_secret)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let opts = use_context::<ResponseOptions>().ok_or_else(|| ServerFnError::new("No response"))?;
    opts.append_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "admin_session={token}; HttpOnly; Path=/; Max-Age=86400; SameSite=Strict"
        ))
        .unwrap(),
    );

    Ok(())
}

#[server(AdminLogout, "/api")]
pub async fn admin_logout() -> Result<(), ServerFnError> {
    use axum::http::{header, HeaderValue};
    use leptos_axum::ResponseOptions;

    let opts = use_context::<ResponseOptions>().ok_or_else(|| ServerFnError::new("No response"))?;
    opts.append_header(
        header::SET_COOKIE,
        HeaderValue::from_str("admin_session=; HttpOnly; Path=/; Max-Age=0; SameSite=Strict")
            .unwrap(),
    );
    Ok(())
}

#[server(CheckAdminAuth, "/api")]
pub async fn check_admin_auth() -> Result<bool, ServerFnError> {
    use axum::http::HeaderMap;
    use super::auth::{extract_session_token, verify_jwt};

    let config = get_config()?;
    let headers: HeaderMap = leptos_axum::extract().await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let cookie = headers.get("cookie").and_then(|v| v.to_str().ok()).unwrap_or("");
    let ok = extract_session_token(cookie)
        .map(|tok| verify_jwt(&tok, &config.jwt_secret).is_ok())
        .unwrap_or(false);
    Ok(ok)
}

// ── MARKET ────────────────────────────────────────────────────────────────────

#[server(GetMarket, "/api")]
pub async fn get_market() -> Result<MarketInfo, ServerFnError> {
    let storage = get_storage()?;
    Ok(storage.get_json_or_default("data/market.json").await.unwrap_or_default())
}

#[server(UpdateMarket, "/api")]
pub async fn update_market(info: MarketInfo) -> Result<MarketInfo, ServerFnError> {
    require_admin().await?;
    let storage = get_storage()?;
    storage.put_json("data/market.json", &info).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(info)
}

// ── MENU ─────────────────────────────────────────────────────────────────────

#[server(GetMenu, "/api")]
pub async fn get_menu() -> Result<MenuData, ServerFnError> {
    let storage = get_storage()?;
    Ok(storage.get_json_or_default("data/menu.json").await.unwrap_or_default())
}

#[server(CreateMenuItem, "/api")]
pub async fn create_menu_item(payload: CreateMenuItemPayload) -> Result<MenuItem, ServerFnError> {
    require_admin().await?;
    if payload.name.trim().is_empty() {
        return Err(ServerFnError::new("Le nom est obligatoire"));
    }
    let storage = get_storage()?;
    let mut data: MenuData = storage.get_json_or_default("data/menu.json").await.unwrap_or_default();

    let item = MenuItem {
        id:          uuid::Uuid::new_v4().to_string(),
        name:        payload.name,
        description: payload.description,
        photo_url:   None,
        category:    payload.category,
        price_info:  payload.price_info,
    };
    data.items.push(item.clone());
    storage.put_json("data/menu.json", &data).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(item)
}

#[server(DeleteMenuItem, "/api")]
pub async fn delete_menu_item(id: String) -> Result<(), ServerFnError> {
    require_admin().await?;
    let storage = get_storage()?;
    let mut data: MenuData = storage.get_json_or_default("data/menu.json").await.unwrap_or_default();
    let len_before = data.items.len();
    data.items.retain(|i| i.id != id);
    if data.items.len() == len_before {
        return Err(ServerFnError::new("Plat introuvable"));
    }
    storage.put_json("data/menu.json", &data).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

// ── DEVIS ────────────────────────────────────────────────────────────────────

#[server(CreateQuote, "/api")]
pub async fn create_quote(payload: CreateQuotePayload) -> Result<(), ServerFnError> {
    use chrono::Utc;

    if payload.main_dish.trim().is_empty() {
        return Err(ServerFnError::new("Le plat principal est obligatoire"));
    }
    if payload.last_name.trim().is_empty() || payload.first_name.trim().is_empty() {
        return Err(ServerFnError::new("Nom et prénom obligatoires"));
    }
    if payload.email.trim().is_empty() {
        return Err(ServerFnError::new("L'email est obligatoire"));
    }
    if payload.number_of_people == 0 {
        return Err(ServerFnError::new("Le nombre de personnes doit être > 0"));
    }

    let storage = get_storage()?;
    let mut data: QuotesData = storage.get_json_or_default("data/quotes.json").await.unwrap_or_default();

    let quote = QuoteRequest {
        id:               uuid::Uuid::new_v4().to_string(),
        last_name:        payload.last_name,
        first_name:       payload.first_name,
        phone:            payload.phone,
        email:            payload.email,
        event_date:       payload.event_date,
        event_place:      payload.event_place,
        number_of_people: payload.number_of_people,
        starters:         payload.starters,
        main_dish:        payload.main_dish,
        desserts:         payload.desserts,
        message:          payload.message,
        created_at:       Utc::now().to_rfc3339(),
        status:           QuoteStatus::Pending,
    };

    data.quotes.push(quote);
    storage.put_json("data/quotes.json", &data).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server(GetQuotes, "/api")]
pub async fn get_quotes() -> Result<QuotesData, ServerFnError> {
    require_admin().await?;
    let storage = get_storage()?;
    Ok(storage.get_json_or_default("data/quotes.json").await.unwrap_or_default())
}
