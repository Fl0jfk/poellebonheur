use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{admin::AdminPage, devis::DevisPage, home::HomePage};

// Remonte automatiquement en haut à chaque changement de page
#[component]
fn ScrollToTop() -> impl IntoView {
    // Désactive la restauration automatique du scroll du navigateur (une seule fois)
    #[cfg(not(feature = "ssr"))]
    {
        if let Some(w) = web_sys::window() {
            if let Ok(hist) = w.history() {
                let _ = hist.set_scroll_restoration(web_sys::ScrollRestoration::Manual);
            }
        }
    }

    let location = use_location();
    create_effect(move |_| {
        let _ = location.pathname.get();
        #[cfg(not(feature = "ssr"))]
        if let Some(w) = web_sys::window() {
            let _ = w.scroll_to_with_x_and_y(0.0, 0.0);
        }
    });
    view! { <></> }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/traiteur-website.css"/>
        <Title text="La Poêlée du Bonheur — Traiteur événementielle"/>
        <Meta name="description" content="La Poêlée du Bonheur — Traiteur paella et cuisine méditerranéenne pour vos événements. Mariages, anniversaires, marchés. Devis gratuit."/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#8b1a1a"/>

        <Router>
            <ScrollToTop/>
            <Routes>
                <Route path="/"       view=HomePage/>
                <Route path="/devis"  view=DevisPage/>
                <Route path="/admin"  view=AdminPage/>
            </Routes>
        </Router>
    }
}
