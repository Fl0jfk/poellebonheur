use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use traiteur_lambdas::{check_admin_key, err, ok, preflight, s3_get, s3_put, MenuItem, MenuData};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct MenuAction {
    action:      String,
    id:          Option<String>,
    name:        Option<String>,
    description: Option<String>,
    category:    Option<String>,
    price_info:  Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();
    run(service_fn(handler)).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if req.method() == lambda_http::http::Method::OPTIONS {
        return Ok(preflight());
    }

    if !check_admin_key(&req) {
        return Ok(err(401, "Non autorisé"));
    }

    let body_str = match req.body() {
        Body::Text(s)   => s.clone(),
        Body::Binary(b) => String::from_utf8(b.to_vec()).unwrap_or_default(),
        Body::Empty     => return Ok(err(400, "Corps vide")),
    };

    let action: MenuAction = match serde_json::from_str(&body_str) {
        Ok(v) => v,
        Err(e) => return Ok(err(400, e)),
    };

    let mut menu: MenuData = s3_get("data/menu.json").await;

    match action.action.as_str() {
        "create" => {
            let name = action.name.unwrap_or_default();
            if name.trim().is_empty() {
                return Ok(err(400, "Le nom est obligatoire"));
            }
            let item = MenuItem {
                id:          Uuid::new_v4().to_string(),
                name,
                description: action.description.unwrap_or_default(),
                photo_url:   None,
                category:    action.category.unwrap_or_else(|| "Starter".to_string()),
                price_info:  action.price_info,
            };
            menu.items.push(item);
            match s3_put("data/menu.json", &menu).await {
                Ok(_)  => Ok(ok(r#"{"success":true}"#)),
                Err(e) => Ok(err(500, e)),
            }
        }

        "delete" => {
            let id = match action.id {
                Some(v) => v,
                None    => return Ok(err(400, "id requis")),
            };
            let before = menu.items.len();
            menu.items.retain(|i| i.id != id);
            if menu.items.len() == before {
                return Ok(err(404, "Plat introuvable"));
            }
            match s3_put("data/menu.json", &menu).await {
                Ok(_)  => Ok(ok(r#"{"success":true}"#)),
                Err(e) => Ok(err(500, e)),
            }
        }

        other => Ok(err(400, format!("Action inconnue : {other}"))),
    }
}
