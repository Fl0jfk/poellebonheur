use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
};
use leptos::LeptosOptions;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use crate::app::App;

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler =
            leptos_axum::render_app_to_stream(options.clone(), move || leptos::view! { <App/> });
        handler(req).await.into_response()
    }
}

async fn get_static_file(
    uri: Uri,
    root: &str,
) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(|b| Body::new(b))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erreur fichiers statiques : {err}"),
        )),
    }
}
