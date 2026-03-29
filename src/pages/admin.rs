use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};

use crate::models::{CreateMenuItemPayload, MarketInfo, MenuCategory, MenuData, MenuItem, QuoteRequest, QuotesData};

// Credentials compilés à la build depuis les variables d'env Vercel
const ADMIN_PASSWORD: &str = match option_env!("ADMIN_PASSWORD") {
    Some(v) => v,
    None    => "admin",
};
const ADMIN_API_KEY: &str = match option_env!("ADMIN_API_KEY") {
    Some(v) => v,
    None    => "",
};

// ── Page admin ────────────────────────────────────────────────────────────────

#[component]
pub fn Admin() -> Element {
    let mut auth_resource = use_resource(|| async move {
        #[cfg(target_arch = "wasm32")]
        { return LocalStorage::get::<bool>("admin_auth").unwrap_or(false); }
        #[allow(unreachable_code)]
        false
    });

    let Some(is_auth) = *auth_resource.read_unchecked() else {
        return rsx! {
            div { class: "min-h-screen flex items-center justify-center",
                div { class: "w-10 h-10 border-4 border-bordeaux-700 border-t-transparent rounded-full animate-spin" }
            }
        };
    };

    if !is_auth {
        rsx! {
            LoginForm {
                on_success: move |_| auth_resource.restart()
            }
        }
    } else {
        rsx! { AdminDashboard {} }
    }
}

// ── Formulaire de connexion ───────────────────────────────────────────────────

#[component]
fn LoginForm(on_success: EventHandler<()>) -> Element {
    let mut password = use_signal(|| String::new());
    let mut loading  = use_signal(|| false);
    let mut error    = use_signal(|| Option::<String>::None);

    rsx! {
        div { class: "min-h-screen flex items-center justify-center px-4",
            div { class: "bg-white rounded-3xl shadow-xl p-10 w-full max-w-sm",
                div { class: "text-center mb-8",
                    div { class: "text-5xl mb-4", "🔐" }
                    h1 { class: "font-display text-2xl text-ardoise-900", "Espace admin" }
                    p { class: "text-ardoise-600 text-sm mt-1", "La Poêlée du Bonheur" }
                }
                form {
                    class: "space-y-5",
                    onsubmit: move |ev| {
                        ev.prevent_default();
                        if loading() { return; }
                        loading.set(true);
                        error.set(None);
                        let pwd = password();
                        if pwd == ADMIN_PASSWORD {
                            #[cfg(target_arch = "wasm32")]
                            { let _ = LocalStorage::set("admin_auth", true); }
                            on_success.call(());
                        } else {
                            error.set(Some("Mot de passe incorrect".to_string()));
                        }
                        loading.set(false);
                    },

                    div {
                        label { class: "form-label", "Mot de passe" }
                        input {
                            r#type: "password",
                            class: "form-input",
                            placeholder: "••••••••",
                            required: true,
                            autofocus: true,
                            value: password(),
                            oninput: move |e| password.set(e.value())
                        }
                    }
                    if let Some(msg) = error() {
                        p { class: "text-red-600 text-sm", "{msg}" }
                    }
                    button {
                        r#type: "submit",
                        class: "btn btn-safran w-full justify-center py-3",
                        disabled: loading(),
                        if loading() { "Connexion..." } else { "Se connecter" }
                    }
                }
            }
        }
    }
}

// ── Dashboard ─────────────────────────────────────────────────────────────────

#[component]
fn AdminDashboard() -> Element {
    let active_tab: Signal<&'static str> = use_signal(|| "quotes");

    rsx! {
        div {
            header { class: "bg-white border-b border-creme-200 sticky top-0 z-30",
                div { class: "max-w-6xl mx-auto px-6 py-4 flex items-center justify-between",
                    h1 { class: "font-display text-xl text-ardoise-900", "Admin — La Poêlée du Bonheur" }
                    button {
                        class: "btn btn-ghost text-sm px-4 py-2",
                        onclick: move |_| {
                            #[cfg(target_arch = "wasm32")]
                            { LocalStorage::delete("admin_auth"); }
                            #[cfg(target_arch = "wasm32")]
                            if let Some(window) = web_sys::window() {
                                let _ = window.location().reload();
                            }
                        },
                        "Déconnexion"
                    }
                }
            }

            div { class: "max-w-6xl mx-auto px-6 pt-8",
                div { class: "flex gap-2 mb-8 border-b border-creme-200",
                    TabButton { label: "Devis", id: "quotes", active: active_tab }
                    TabButton { label: "Menu", id: "menu", active: active_tab }
                    TabButton { label: "Marché", id: "market", active: active_tab }
                }

                match active_tab() {
                    "quotes" => rsx! { QuotesPanel {} },
                    "menu"   => rsx! { MenuPanel {} },
                    "market" => rsx! { MarketPanel {} },
                    _        => rsx! {},
                }
            }
        }
    }
}

#[component]
fn TabButton(label: &'static str, id: &'static str, mut active: Signal<&'static str>) -> Element {
    let is_active = active() == id;
    rsx! {
        button {
            class: if is_active {
                "px-4 py-2.5 text-sm font-semibold text-bordeaux-700 border-b-2 border-bordeaux-700 -mb-px"
            } else {
                "px-4 py-2.5 text-sm font-medium text-ardoise-600 hover:text-bordeaux-700 border-b-2 border-transparent -mb-px"
            },
            onclick: move |_| active.set(id),
            "{label}"
        }
    }
}

// ── Onglet Devis ─────────────────────────────────────────────────────────────

#[component]
fn QuotesPanel() -> Element {
    let s3_base = "https://poellebonheur.s3.eu-west-3.amazonaws.com";
    let mut quotes_resource = use_resource(move || async move {
        let url = format!("{s3_base}/data/quotes.json");
        reqwest::get(&url).await.ok()?.json::<QuotesData>().await.ok()
    });

    rsx! {
        div { class: "pb-16",
            div { class: "flex items-center justify-between mb-6",
                h2 { class: "font-display text-2xl text-ardoise-900", "Demandes de devis" }
                button {
                    class: "btn btn-ghost text-sm px-4 py-2",
                    onclick: move |_| quotes_resource.restart(),
                    "Actualiser"
                }
            }

            {
                let q_ref = quotes_resource.read();
                match q_ref.as_ref() {
                    None => rsx! {
                        div { class: "flex justify-center py-12",
                            div { class: "w-8 h-8 border-4 border-bordeaux-700 border-t-transparent rounded-full animate-spin" }
                        }
                    },
                    Some(None) => rsx! {
                        p { class: "text-ardoise-500 py-4 italic", "Aucune demande de devis pour le moment." }
                    },
                    Some(Some(data)) => {
                        let mut sorted = data.quotes.clone();
                        sorted.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                        if sorted.is_empty() {
                            rsx! {
                                div { class: "text-center py-16 text-ardoise-500",
                                    div { class: "text-5xl mb-4", "📭" }
                                    p { "Aucune demande pour le moment." }
                                }
                            }
                        } else {
                            rsx! {
                                div { class: "space-y-4",
                                    for q in sorted {
                                        QuoteCard { q }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn QuoteCard(q: QuoteRequest) -> Element {
    let date_short  = q.created_at.get(..10).unwrap_or(&q.created_at).to_string();
    let event_date  = if q.event_date.is_empty() { "Date non précisée".to_string() } else { q.event_date.clone() };
    let starters    = q.starters.join(", ");
    let desserts    = q.desserts.join(", ");
    let full_name   = format!("{} {}", q.first_name, q.last_name);
    let people      = q.number_of_people.to_string();

    rsx! {
        div { class: "bg-white rounded-2xl shadow-sm border border-creme-200 p-6",
            div { class: "flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3 mb-4",
                div {
                    h3 { class: "font-semibold text-ardoise-900 text-lg", "{full_name}" }
                    p { class: "text-ardoise-500 text-sm", "{date_short}" }
                }
            }
            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-3 text-sm mb-4",
                div {
                    span { class: "text-ardoise-500", "📧 " }
                    a { href: "mailto:{q.email}", class: "text-bordeaux-600 hover:underline", "{q.email}" }
                }
                if !q.phone.is_empty() {
                    div {
                        span { class: "text-ardoise-500", "📞 " }
                        a { href: "tel:{q.phone}", class: "text-bordeaux-600 hover:underline", "{q.phone}" }
                    }
                }
                div { "📅 {event_date}" }
                div { "👥 {people} personnes" }
                if !q.event_place.is_empty() {
                    div { class: "sm:col-span-2", "📍 {q.event_place}" }
                }
            }
            div { class: "border-t border-creme-100 pt-3 text-sm space-y-1",
                if !starters.is_empty() {
                    p { span { class: "text-ardoise-500 font-medium", "Entrées : " } "{starters}" }
                }
                if !q.main_dish.is_empty() {
                    p { span { class: "text-ardoise-500 font-medium", "Plat : " } "{q.main_dish}" }
                }
                if !desserts.is_empty() {
                    p { span { class: "text-ardoise-500 font-medium", "Desserts : " } "{desserts}" }
                }
                if let Some(m) = q.message {
                    p { class: "text-ardoise-600 italic mt-2", "{m}" }
                }
            }
        }
    }
}

// ── Onglet Menu ───────────────────────────────────────────────────────────────

#[component]
fn MenuPanel() -> Element {
    let s3_base = "https://poellebonheur.s3.eu-west-3.amazonaws.com";
    let mut menu_resource = use_resource(move || async move {
        let url = format!("{s3_base}/data/menu.json");
        reqwest::get(&url).await.ok()?.json::<MenuData>().await.ok()
    });

    let mut name        = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut category    = use_signal(|| "starter".to_string());
    let mut price_info  = use_signal(|| String::new());
    let mut form_error  = use_signal(|| Option::<String>::None);
    let mut saving      = use_signal(|| false);

    rsx! {
        div { class: "pb-16 space-y-10",
            div { class: "bg-white rounded-2xl shadow-sm border border-creme-200 p-6",
                h2 { class: "font-display text-xl text-ardoise-900 mb-5", "Ajouter un plat" }
                form {
                    class: "space-y-4",
                    onsubmit: move |ev| {
                        ev.prevent_default();
                        if saving() { return; }
                        let cat_str = match category().as_str() {
                            "main_dish" => "MainDish",
                            "dessert"   => "Dessert",
                            _           => "Starter",
                        };
                        let payload = CreateMenuItemPayload {
                            name:        name(),
                            description: description(),
                            category:    match cat_str {
                                "MainDish" => MenuCategory::MainDish,
                                "Dessert"  => MenuCategory::Dessert,
                                _          => MenuCategory::Starter,
                            },
                            price_info:  if price_info().is_empty() { None } else { Some(price_info()) },
                        };
                        saving.set(true);
                        form_error.set(None);
                        spawn(async move {
                            match api_create_menu_item(payload).await {
                                Ok(_) => {
                                    name.set(String::new());
                                    description.set(String::new());
                                    price_info.set(String::new());
                                    menu_resource.restart();
                                }
                                Err(e) => form_error.set(Some(e)),
                            }
                            saving.set(false);
                        });
                    },

                    div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                        div {
                            label { class: "form-label", "Nom *" }
                            input { r#type: "text", class: "form-input", required: true,
                                value: name(), oninput: move |e| name.set(e.value()) }
                        }
                        div {
                            label { class: "form-label", "Catégorie" }
                            select {
                                class: "form-input",
                                onchange: move |e| category.set(e.value()),
                                option { value: "starter", "Entrée" }
                                option { value: "main_dish", "Plat principal" }
                                option { value: "dessert", "Dessert" }
                            }
                        }
                    }
                    div {
                        label { class: "form-label", "Description" }
                        input { r#type: "text", class: "form-input",
                            value: description(), oninput: move |e| description.set(e.value()) }
                    }
                    div {
                        label { class: "form-label", "Info prix (optionnel)" }
                        input { r#type: "text", class: "form-input", placeholder: "ex: 8€/pers.",
                            value: price_info(), oninput: move |e| price_info.set(e.value()) }
                    }
                    if let Some(msg) = form_error() {
                        p { class: "text-red-600 text-sm", "{msg}" }
                    }
                    button {
                        r#type: "submit",
                        class: "btn btn-safran px-6",
                        disabled: saving(),
                        if saving() { "Ajout..." } else { "Ajouter le plat" }
                    }
                }
            }

            div {
                h2 { class: "font-display text-xl text-ardoise-900 mb-5", "Carte actuelle" }
                {
                    let menu_ref = menu_resource.read();
                    match menu_ref.as_ref().and_then(|o| o.as_ref()) {
                        None => rsx! {
                            div { class: "flex justify-center py-8",
                                div { class: "w-8 h-8 border-4 border-bordeaux-700 border-t-transparent rounded-full animate-spin" }
                            }
                        },
                        Some(data) if data.items.is_empty() => rsx! {
                            p { class: "text-ardoise-500 text-sm", "Aucun plat dans la carte." }
                        },
                        Some(data) => {
                            let items = data.items.clone();
                            rsx! {
                                div { class: "space-y-2",
                                    for item in items {
                                        MenuItemRow { item, on_delete: move |id: String| {
                                            spawn(async move {
                                                if api_delete_menu_item(id).await.is_ok() {
                                                    menu_resource.restart();
                                                }
                                            });
                                        }}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MenuItemRow(item: MenuItem, on_delete: EventHandler<String>) -> Element {
    let item_id = item.id.clone();
    rsx! {
        div { class: "bg-white rounded-xl border border-creme-200 px-5 py-4 flex items-center justify-between gap-4",
            div { class: "flex-1 min-w-0",
                div { class: "flex items-center gap-2 mb-0.5",
                    span { class: "font-medium text-ardoise-900", "{item.name}" }
                    span { class: "tag bg-creme-100 text-ardoise-600 text-xs", "{item.category.label()}" }
                    if let Some(p) = item.price_info {
                        span { class: "tag bg-safran-100 text-safran-700 text-xs", "{p}" }
                    }
                }
                p { class: "text-ardoise-500 text-sm truncate", "{item.description}" }
            }
            button {
                class: "text-red-400 hover:text-red-600 transition-colors p-1.5 rounded-lg hover:bg-red-50 flex-shrink-0",
                title: "Supprimer",
                onclick: move |_| on_delete.call(item_id.clone()),
                svg { class: "w-4 h-4", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" }
                }
            }
        }
    }
}

// ── Onglet Marché ─────────────────────────────────────────────────────────────

#[component]
fn MarketPanel() -> Element {
    let s3_base = "https://poellebonheur.s3.eu-west-3.amazonaws.com";

    let mut date    = use_signal(|| String::new());
    let mut place   = use_signal(|| String::new());
    let mut active  = use_signal(|| false);
    let mut saving  = use_signal(|| false);
    let mut saved   = use_signal(|| false);
    let mut err_msg = use_signal(|| Option::<String>::None);
    let mut loaded  = use_signal(|| false);

    let _load = use_resource(move || async move {
        let url = format!("{s3_base}/data/market.json");
        if let Ok(resp) = reqwest::get(&url).await {
            if let Ok(m) = resp.json::<MarketInfo>().await {
                if !loaded() {
                    date.set(m.date.unwrap_or_default());
                    place.set(m.place.unwrap_or_default());
                    active.set(m.active);
                    loaded.set(true);
                }
            }
        }
    });

    rsx! {
        div { class: "pb-16 max-w-lg",
            h2 { class: "font-display text-2xl text-ardoise-900 mb-6", "Prochain marché" }
            div { class: "bg-white rounded-2xl shadow-sm border border-creme-200 p-6",
                form {
                    class: "space-y-5",
                    onsubmit: move |ev| {
                        ev.prevent_default();
                        if saving() { return; }
                        saving.set(true);
                        err_msg.set(None);
                        saved.set(false);
                        let info = MarketInfo {
                            date:   if date().is_empty() { None } else { Some(date()) },
                            place:  if place().is_empty() { None } else { Some(place()) },
                            active: active(),
                        };
                        spawn(async move {
                            match api_update_market(info).await {
                                Ok(_)  => saved.set(true),
                                Err(e) => err_msg.set(Some(e)),
                            }
                            saving.set(false);
                        });
                    },

                    div {
                        label { class: "form-label", "Date" }
                        input { r#type: "text", class: "form-input", placeholder: "ex: Samedi 5 avril 2025",
                            value: date(), oninput: move |e| date.set(e.value()) }
                    }
                    div {
                        label { class: "form-label", "Lieu" }
                        input { r#type: "text", class: "form-input", placeholder: "ex: Marché de Montrouge",
                            value: place(), oninput: move |e| place.set(e.value()) }
                    }
                    label { class: "flex items-center gap-3 cursor-pointer",
                        input {
                            r#type: "checkbox",
                            class: "w-5 h-5 rounded accent-bordeaux-700",
                            checked: active(),
                            onchange: move |_| active.set(!active())
                        }
                        span { class: "text-sm font-medium text-ardoise-800", "Afficher sur la page d'accueil" }
                    }
                    if let Some(msg) = err_msg() {
                        p { class: "text-red-600 text-sm", "{msg}" }
                    }
                    if saved() {
                        p { class: "text-green-600 text-sm", "✓ Enregistré avec succès" }
                    }
                    button {
                        r#type: "submit",
                        class: "btn btn-safran px-6",
                        disabled: saving(),
                        if saving() { "Enregistrement..." } else { "Enregistrer" }
                    }
                }
            }
        }
    }
}

// ── Appels aux Vercel Functions ───────────────────────────────────────────────

fn admin_client() -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    if !ADMIN_API_KEY.is_empty() {
        if let Ok(v) = reqwest::header::HeaderValue::from_str(ADMIN_API_KEY) {
            headers.insert("x-admin-key", v);
        }
    }
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap_or_default()
}

async fn api_update_market(info: MarketInfo) -> Result<(), String> {
    let resp = admin_client()
        .post("/api/admin/market")
        .json(&info)
        .send()
        .await
        .map_err(|e| format!("Erreur réseau : {e}"))?;

    if resp.status().is_success() { Ok(()) }
    else { Err(format!("Erreur {}", resp.status())) }
}

async fn api_create_menu_item(payload: CreateMenuItemPayload) -> Result<(), String> {
    let body = serde_json::json!({
        "action":      "create",
        "name":        payload.name,
        "description": payload.description,
        "category":    format!("{:?}", payload.category),
        "price_info":  payload.price_info,
    });
    let resp = admin_client()
        .post("/api/admin/menu")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Erreur réseau : {e}"))?;

    if resp.status().is_success() { Ok(()) }
    else {
        let msg = resp.text().await.unwrap_or_else(|_| "Erreur serveur".into());
        Err(msg)
    }
}

async fn api_delete_menu_item(id: String) -> Result<(), String> {
    let body = serde_json::json!({ "action": "delete", "id": id });
    let resp = admin_client()
        .post("/api/admin/menu")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Erreur réseau : {e}"))?;

    if resp.status().is_success() { Ok(()) }
    else { Err(format!("Erreur {}", resp.status())) }
}
