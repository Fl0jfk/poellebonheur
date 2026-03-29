use dioxus::prelude::*;
use crate::models::*;

// ── Helper S3 (server uniquement) ────────────────────────────────────────────

#[cfg(feature = "server")]
async fn s3_client() -> aws_sdk_s3::Client {
    use aws_config::Region;
    use aws_sdk_s3::config::Credentials;

    let key_id = std::env::var("ACCESS_KEY_ID")
        .or_else(|_| std::env::var("AWS_ACCESS_KEY_ID"))
        .expect("ACCESS_KEY_ID manquant");
    let secret = std::env::var("SECRET_ACCESS_KEY")
        .or_else(|_| std::env::var("AWS_SECRET_ACCESS_KEY"))
        .expect("SECRET_ACCESS_KEY manquant");
    let region = std::env::var("AWS_REGION").unwrap_or_else(|_| "eu-west-3".to_string());

    let credentials = Credentials::new(key_id, secret, None, None, "manual");
    let config = aws_config::from_env()
        .region(Region::new(region))
        .credentials_provider(credentials)
        .load()
        .await;

    aws_sdk_s3::Client::new(&config)
}

#[cfg(feature = "server")]
fn bucket() -> String {
    std::env::var("S3_BUCKET_NAME").unwrap_or_else(|_| "poellebonheur".to_string())
}

#[cfg(feature = "server")]
async fn s3_get<T: serde::de::DeserializeOwned + Default>(key: &str) -> T {
    let client = s3_client().await;
    let resp = client.get_object().bucket(bucket()).key(key).send().await;
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

#[cfg(feature = "server")]
async fn s3_put<T: serde::Serialize>(key: &str, data: &T) -> Result<(), ServerFnError> {
    use aws_sdk_s3::primitives::ByteStream;
    let client = s3_client().await;
    let json = serde_json::to_vec(data)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    client
        .put_object()
        .bucket(bucket())
        .key(key)
        .body(ByteStream::from(json))
        .content_type("application/json")
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

// ── AUTH ─────────────────────────────────────────────────────────────────────

#[server]
pub async fn admin_login(password: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let hash = std::env::var("ADMIN_PASSWORD_HASH")
            .unwrap_or_else(|_| std::env::var("ADMIN_PASSWORD").unwrap_or_default());

        // Support bcrypt hash et mot de passe en clair (développement)
        let valid = if hash.starts_with("$2") {
            bcrypt::verify(&password, &hash).unwrap_or(false)
        } else {
            password == hash
        };
        return Ok(valid);
    }
    #[allow(unreachable_code)]
    Ok(false)
}

// ── MARKET ────────────────────────────────────────────────────────────────────

#[server]
pub async fn update_market(info: MarketInfo) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        s3_put("data/market.json", &info).await?;
        return Ok(());
    }
    #[allow(unreachable_code)]
    Ok(())
}

// ── MENU ─────────────────────────────────────────────────────────────────────

#[server]
pub async fn create_menu_item(payload: CreateMenuItemPayload) -> Result<MenuItem, ServerFnError> {
    #[cfg(feature = "server")]
    {
        if payload.name.trim().is_empty() {
            return Err(ServerFnError::new("Le nom est obligatoire"));
        }
        let mut data: MenuData = s3_get("data/menu.json").await;
        let item = MenuItem {
            id:          uuid::Uuid::new_v4().to_string(),
            name:        payload.name,
            description: payload.description,
            photo_url:   None,
            category:    payload.category,
            price_info:  payload.price_info,
        };
        data.items.push(item.clone());
        s3_put("data/menu.json", &data).await?;
        return Ok(item);
    }
    #[allow(unreachable_code)]
    Err(ServerFnError::new("Unavailable"))
}

#[server]
pub async fn delete_menu_item(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        let mut data: MenuData = s3_get("data/menu.json").await;
        let len_before = data.items.len();
        data.items.retain(|i| i.id != id);
        if data.items.len() == len_before {
            return Err(ServerFnError::new("Plat introuvable"));
        }
        s3_put("data/menu.json", &data).await?;
        return Ok(());
    }
    #[allow(unreachable_code)]
    Ok(())
}

// ── DEVIS ────────────────────────────────────────────────────────────────────

#[server]
pub async fn create_quote(payload: CreateQuotePayload) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
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

        let mut data: QuotesData = s3_get("data/quotes.json").await;
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
        s3_put("data/quotes.json", &data).await?;
        return Ok(());
    }
    #[allow(unreachable_code)]
    Ok(())
}
