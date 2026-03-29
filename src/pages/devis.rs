use dioxus::prelude::*;
use crate::components::{footer::Footer, navbar::Navbar};
use crate::models::{CreateQuotePayload, MenuCategory, MenuData};

#[component]
pub fn Devis() -> Element {
    let s3_base = "https://poellebonheur.s3.eu-west-3.amazonaws.com";

    let menu = use_resource(move || async move {
        let url = format!("{s3_base}/data/menu.json");
        reqwest::get(&url).await.ok()?.json::<MenuData>().await.ok()
    });

    let mut last_name        = use_signal(|| String::new());
    let mut first_name       = use_signal(|| String::new());
    let mut phone            = use_signal(|| String::new());
    let mut email            = use_signal(|| String::new());
    let mut event_date       = use_signal(|| String::new());
    let mut event_place      = use_signal(|| String::new());
    let mut number_of_people = use_signal(|| String::from("10"));
    let mut main_dish        = use_signal(|| String::new());
    let mut starters         = use_signal(|| Vec::<String>::new());
    let mut desserts         = use_signal(|| Vec::<String>::new());
    let mut message          = use_signal(|| String::new());
    let mut submitting       = use_signal(|| false);
    let mut success          = use_signal(|| false);
    let mut error_msg        = use_signal(|| Option::<String>::None);

    rsx! {
        div { class: "min-h-screen flex flex-col",
        Navbar {}
        main { class: "flex-1 pt-24 pb-16",
            div { class: "max-w-3xl mx-auto px-6",

                div { class: "text-center mb-12",
                    span { class: "section-label text-safran-600 block mb-3", "Gratuit & sans engagement" }
                    h1 { class: "font-display text-4xl md:text-5xl text-ardoise-800 mb-4", "Votre devis 🍽️" }
                    p { class: "font-body text-ardoise-500 max-w-xl mx-auto",
                        "Décrivez votre événement et composez votre menu. On vous répond rapidement avec une proposition personnalisée."
                    }
                }

                if success() {
                    div { class: "bg-green-50 border border-green-200 rounded-3xl p-10 text-center animate-fade-in",
                        div { class: "text-6xl mb-4", "🥘🎉" }
                        h2 { class: "font-display text-3xl text-green-800 mb-2", "Votre demande est envoyée !" }
                        p { class: "font-body text-green-700", "On vous recontacte dans les plus brefs délais." }
                    }
                } else {
                    form {
                        class: "bg-white rounded-3xl shadow-lg p-8 space-y-8",
                        onsubmit: move |ev| {
                            ev.prevent_default();
                            if submitting() { return; }
                            let n: u32 = number_of_people().parse().unwrap_or(0);
                            if n == 0 {
                                error_msg.set(Some("Veuillez indiquer un nombre de personnes valide.".into()));
                                return;
                            }
                            if main_dish().is_empty() {
                                error_msg.set(Some("Veuillez choisir un plat principal.".into()));
                                return;
                            }
                            submitting.set(true);
                            error_msg.set(None);
                            let payload = CreateQuotePayload {
                                last_name:        last_name(),
                                first_name:       first_name(),
                                phone:            phone(),
                                email:            email(),
                                event_date:       event_date(),
                                event_place:      event_place(),
                                number_of_people: n,
                                starters:         starters(),
                                main_dish:        main_dish(),
                                desserts:         desserts(),
                                message:          if message().is_empty() { None } else { Some(message()) },
                            };
                            spawn(async move {
                                match send_devis(&payload).await {
                                    Ok(_)  => success.set(true),
                                    Err(e) => error_msg.set(Some(e)),
                                }
                                submitting.set(false);
                            });
                        },

                        // Coordonnées
                        fieldset { class: "space-y-5",
                            legend { class: "font-hand text-2xl font-bold text-ardoise-800 mb-1", "Vos coordonnées" }
                            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                                div {
                                    label { class: "form-label", "Nom *" }
                                    input { r#type: "text", class: "form-input", placeholder: "Dupont", required: true,
                                        value: last_name(), oninput: move |e| last_name.set(e.value()) }
                                }
                                div {
                                    label { class: "form-label", "Prénom *" }
                                    input { r#type: "text", class: "form-input", placeholder: "Marie", required: true,
                                        value: first_name(), oninput: move |e| first_name.set(e.value()) }
                                }
                            }
                            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                                div {
                                    label { class: "form-label", "Téléphone" }
                                    input { r#type: "tel", class: "form-input", placeholder: "06 00 00 00 00",
                                        value: phone(), oninput: move |e| phone.set(e.value()) }
                                }
                                div {
                                    label { class: "form-label", "Email *" }
                                    input { r#type: "email", class: "form-input", placeholder: "marie@example.fr", required: true,
                                        value: email(), oninput: move |e| email.set(e.value()) }
                                }
                            }
                        }

                        hr { class: "border-creme-200" }

                        // Événement
                        fieldset { class: "space-y-5",
                            legend { class: "font-hand text-2xl font-bold text-ardoise-800 mb-1", "Votre événement" }
                            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                                div {
                                    label { class: "form-label", "Date de l'événement" }
                                    input { r#type: "date", class: "form-input",
                                        value: event_date(), oninput: move |e| event_date.set(e.value()) }
                                }
                                div {
                                    label { class: "form-label", "Nombre de personnes *" }
                                    input { r#type: "number", class: "form-input", min: "1", required: true,
                                        value: number_of_people(), oninput: move |e| number_of_people.set(e.value()) }
                                }
                            }
                            div {
                                label { class: "form-label", "Lieu de l'événement" }
                                input { r#type: "text", class: "form-input", placeholder: "Salle des fêtes, Paris 75001...",
                                    value: event_place(), oninput: move |e| event_place.set(e.value()) }
                            }
                        }

                        hr { class: "border-creme-200" }

                        // Composition du menu
                        fieldset {
                            legend { class: "font-hand text-2xl font-bold text-ardoise-800 mb-4", "Composition du menu 🥘" }
                            {
                                let menu_ref = menu.read();
                                match menu_ref.as_ref().and_then(|o| o.as_ref()) {
                                    None => rsx! {
                                        div { class: "flex justify-center py-8",
                                            div { class: "w-8 h-8 border-4 border-bordeaux-700 border-t-transparent rounded-full animate-spin" }
                                        }
                                    },
                                    Some(data) if data.items.is_empty() => rsx! {
                                        p { class: "text-ardoise-500 text-sm italic py-4", "La carte n'est pas encore disponible. Précisez vos souhaits dans le message." }
                                    },
                                    Some(data) => {
                                        let starter_items: Vec<_> = data.items.iter().filter(|i| i.category == MenuCategory::Starter).cloned().collect();
                                        let main_items: Vec<_>    = data.items.iter().filter(|i| i.category == MenuCategory::MainDish).cloned().collect();
                                        let dessert_items: Vec<_> = data.items.iter().filter(|i| i.category == MenuCategory::Dessert).cloned().collect();
                                        rsx! {
                                            div { class: "space-y-6",
                                                if !starter_items.is_empty() {
                                                    div {
                                                        p { class: "form-label", "🥗 Entrées (choix multiples)" }
                                                        div { class: "flex flex-wrap gap-2",
                                                            for item in starter_items {
                                                                {
                                                                    let id = item.id.clone();
                                                                    let id2 = id.clone();
                                                                    let is_selected = starters().contains(&id);
                                                                    rsx! {
                                                                        button {
                                                                            key: "{id}",
                                                                            r#type: "button",
                                                                            class: if is_selected { "tag bg-bordeaux-600 text-white cursor-pointer" } else { "tag bg-creme-100 text-ardoise-700 cursor-pointer hover:bg-bordeaux-100" },
                                                                            onclick: move |_| starters.with_mut(|v| { if v.contains(&id2) { v.retain(|x| x != &id2); } else { v.push(id2.clone()); } }),
                                                                            "{item.name}"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                if !main_items.is_empty() {
                                                    div {
                                                        p { class: "form-label", "🍲 Plat principal *" }
                                                        div { class: "flex flex-wrap gap-2",
                                                            for item in main_items {
                                                                {
                                                                    let id = item.id.clone();
                                                                    let id2 = id.clone();
                                                                    let is_selected = main_dish() == id;
                                                                    rsx! {
                                                                        button {
                                                                            key: "{id}",
                                                                            r#type: "button",
                                                                            class: if is_selected { "tag bg-bordeaux-700 text-white cursor-pointer" } else { "tag bg-creme-100 text-ardoise-700 cursor-pointer hover:bg-bordeaux-100" },
                                                                            onclick: move |_| main_dish.set(id2.clone()),
                                                                            "{item.name}"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                if !dessert_items.is_empty() {
                                                    div {
                                                        p { class: "form-label", "🍮 Desserts (choix multiples)" }
                                                        div { class: "flex flex-wrap gap-2",
                                                            for item in dessert_items {
                                                                {
                                                                    let id = item.id.clone();
                                                                    let id2 = id.clone();
                                                                    let is_selected = desserts().contains(&id);
                                                                    rsx! {
                                                                        button {
                                                                            key: "{id}",
                                                                            r#type: "button",
                                                                            class: if is_selected { "tag bg-safran-500 text-white cursor-pointer" } else { "tag bg-creme-100 text-ardoise-700 cursor-pointer hover:bg-safran-100" },
                                                                            onclick: move |_| desserts.with_mut(|v| { if v.contains(&id2) { v.retain(|x| x != &id2); } else { v.push(id2.clone()); } }),
                                                                            "{item.name}"
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
                                }
                            }
                        }

                        hr { class: "border-creme-200" }

                        // Message
                        div {
                            label { class: "form-label", "Message complémentaire" }
                            textarea {
                                class: "form-input min-h-[100px] resize-y",
                                placeholder: "Allergies, demandes particulières, style de service...",
                                value: message(),
                                oninput: move |e| message.set(e.value())
                            }
                        }

                        if let Some(msg) = error_msg() {
                            div { class: "bg-red-50 border border-red-200 text-red-700 rounded-xl px-4 py-3 text-sm", "{msg}" }
                        }

                        button {
                            r#type: "submit",
                            class: "btn btn-safran w-full justify-center text-base py-4 shadow-lg",
                            disabled: submitting(),
                            if submitting() { "Envoi en cours..." } else { "🍽️ Envoyer ma demande de devis" }
                        }
                    }
                }
            }
        }
        Footer {}
        }
    }
}

async fn send_devis(payload: &CreateQuotePayload) -> Result<(), String> {
    let body = serde_json::json!({
        "last_name":        payload.last_name,
        "first_name":       payload.first_name,
        "phone":            payload.phone,
        "email":            payload.email,
        "event_date":       payload.event_date,
        "event_place":      payload.event_place,
        "number_of_people": payload.number_of_people,
        "starters":         payload.starters,
        "main_dish":        payload.main_dish,
        "desserts":         payload.desserts,
        "message":          payload.message,
    });

    let url = format!("{}/send-devis", crate::config::API_BASE);
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Erreur réseau : {e}"))?;

    if resp.status().is_success() {
        Ok(())
    } else {
        let msg = resp.text().await.unwrap_or_else(|_| "Erreur serveur".into());
        Err(format!("Erreur : {msg}"))
    }
}
