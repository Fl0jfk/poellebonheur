#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::post, Extension, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::sync::Arc;
    use traiteur_website::{
        app::App,
        fileserv::file_and_error_handler,
        server::{config::Config, storage::Storage, upload_photo_handler},
    };

    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            ),
        )
        .init();

    let config = Arc::new(Config::from_env().expect("Configuration invalide"));
    let storage = Arc::new(Storage::new(&config).await.expect("Erreur connexion S3"));

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options.clone();
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        // Upload de photo (multipart — hors server functions Leptos)
        .route("/api/admin/photo/:id", post(upload_photo_handler))
        // Server functions Leptos avec contexte injecté
        .route(
            "/api/*fn_name",
            post({
                let s = storage.clone();
                let c = config.clone();
                move |req| {
                    let s = s.clone();
                    let c = c.clone();
                    async move {
                        leptos_axum::handle_server_fns_with_context(
                            move || {
                                provide_context(s.clone());
                                provide_context(c.clone());
                            },
                            req,
                        )
                        .await
                    }
                }
            }),
        )
        // Routes Leptos SSR avec contexte
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let s = storage.clone();
                let c = config.clone();
                move || {
                    provide_context(s.clone());
                    provide_context(c.clone());
                }
            },
            App,
        )
        .fallback(file_and_error_handler)
        // Extensions pour le handler upload (hors Leptos context)
        .layer(Extension(storage.clone()))
        .layer(Extension(config.clone()))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("🍲 Serveur Nusha Traiteur démarré sur http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {}
