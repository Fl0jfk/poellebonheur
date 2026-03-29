use dioxus::prelude::*;

use crate::pages::{admin::Admin, devis::Devis, home::Home};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/devis")]
    Devis {},
    #[route("/admin")]
    Admin {},
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: "/tailwind.css" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        document::Meta { name: "description", content: "La Poêlée du Bonheur — Traiteur paella et cuisine méditerranéenne pour vos événements. Mariages, anniversaires, marchés. Devis gratuit." }
        document::Title { "La Poêlée du Bonheur — Traiteur événementielle" }
        Router::<Route> {}
    }
}
