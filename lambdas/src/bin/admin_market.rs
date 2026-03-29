use lambda_http::{run, service_fn, Body, Error, Request, Response};
use traiteur_lambdas::{check_admin_key, err, ok, preflight, s3_put, MarketInfo};

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

    let info: MarketInfo = match serde_json::from_str(&body_str) {
        Ok(v) => v,
        Err(e) => return Ok(err(400, e)),
    };

    match s3_put("data/market.json", &info).await {
        Ok(_)  => Ok(ok(r#"{"success":true}"#)),
        Err(e) => Ok(err(500, e)),
    }
}
