use dioxus::prelude::*;
use crate::app::Route;
use crate::components::{footer::Footer, navbar::Navbar};
use crate::models::{MarketInfo, MenuCategory, MenuData};

// ── Pêle-mêle photos ─────────────────────────────────────────────────────────

#[component]
fn PhotoCollage() -> Element {
    rsx! {
        div { class: "collage-wrapper",
            input { r#type: "radio", name: "collage", id: "cp0", checked: true }
            input { r#type: "radio", name: "collage", id: "cp1" }
            input { r#type: "radio", name: "collage", id: "cp2" }
            input { r#type: "radio", name: "collage", id: "cp3" }
            input { r#type: "radio", name: "collage", id: "cp4" }

            label { r#for: "cp0", class: "collage-photo collage-p0",
                img { src: "/Plat1.jpg", alt: "Paella géante", style: "width:100%;height:100%;object-fit:cover;display:block;pointer-events:none;" }
            }
            label { r#for: "cp1", class: "collage-photo collage-p1",
                img { src: "/Entree1.jpg", alt: "Fruits de mer", style: "width:100%;height:100%;object-fit:cover;display:block;pointer-events:none;" }
            }
            label { r#for: "cp2", class: "collage-photo collage-p2",
                img { src: "/Dessert1.jpg", alt: "Desserts maison", style: "width:100%;height:100%;object-fit:cover;display:block;pointer-events:none;" }
            }
            label { r#for: "cp3", class: "collage-photo collage-p3",
                img { src: "/Repas2.jpg", alt: "Repas convivial", style: "width:100%;height:100%;object-fit:cover;display:block;pointer-events:none;" }
            }
            label { r#for: "cp4", class: "collage-photo collage-p4",
                img { src: "/Repas3.jpg", alt: "Repas festif", style: "width:100%;height:100%;object-fit:cover;display:block;pointer-events:none;" }
            }
        }
    }
}

// ── Page d'accueil ───────────────────────────────────────────────────────────

#[component]
pub fn Home() -> Element {
    let s3_base = "https://poellebonheur.s3.eu-west-3.amazonaws.com";

    let market = use_resource(move || async move {
        let url = format!("{s3_base}/data/market.json");
        reqwest::get(&url).await.ok()?.json::<MarketInfo>().await.ok()
    });

    let menu = use_resource(move || async move {
        let url = format!("{s3_base}/data/menu.json");
        reqwest::get(&url).await.ok()?.json::<MenuData>().await.ok()
    });

    rsx! {
        div { class: "min-h-screen flex flex-col",
        Navbar {}
        // ── Hero ──────────────────────────────────────────────────────────
        section { class: "relative min-h-screen flex items-center justify-center pt-16",

            // Fond décoratif (layer séparé)
            div { class: "absolute inset-0 overflow-hidden",
                div { class: "absolute inset-0 bg-gradient-to-br from-ardoise-700 via-ardoise-800 to-ardoise-900" }
                div {
                    class: "absolute inset-0 opacity-5",
                    style: "background-image: radial-gradient(circle, #e8a030 1px, transparent 1px); background-size: 40px 40px;"
                }
                div { class: "absolute top-20 left-10 w-72 h-72 bg-bordeaux-700/20 rounded-full blur-3xl" }
                div { class: "absolute bottom-20 right-10 w-96 h-96 bg-safran-500/15 rounded-full blur-3xl" }
            }

            div { class: "relative z-10 text-center w-full max-w-4xl mx-auto",
                div { class: "flex justify-center mb-8 sm:mb-12",
                    PhotoCollage {}
                }

                div { class: "px-6",
                    h1 { class: "font-display text-5xl md:text-7xl text-white mb-4 drop-shadow-lg",
                        "La Poêlée"
                        br {}
                        span { class: "text-safran-400", "du Bonheur" }
                    }
                    p { class: "font-hand text-2xl text-ardoise-300 mb-3", "Traiteur événementielle" }
                    p { class: "font-body text-ardoise-300 text-lg max-w-2xl mx-auto mb-10 leading-relaxed",
                        "Paella géante, fruits de mer et saveurs méditerranéennes cuisinés avec passion pour vos mariages, anniversaires et marchés. 🦐🥘"
                    }
                    div { class: "flex flex-col sm:flex-row gap-4 justify-center items-center",
                        Link { to: Route::Devis {}, class: "btn btn-safran text-base px-10 py-4 shadow-lg", "🍽️ Demander un devis" }
                        a { href: "#menu", class: "btn btn-ghost text-base px-8 py-4", "Voir notre carte" }
                    }
                }
            }

            div { class: "absolute bottom-8 xl:bottom-4 left-1/2 -translate-x-1/2 animate-bounce",
                svg { class: "w-6 h-6 text-white/40", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M19 9l-7 7-7-7" }
                }
            }
        }

        // ── Bandeau marché ────────────────────────────────────────────────
        {
            let market_ref = market.read();
            if let Some(Some(m)) = market_ref.as_ref() {
                if m.active {
                    let date = m.date.clone().unwrap_or_default();
                    let place = m.place.clone().unwrap_or_default();
                    rsx! {
                        div { class: "bg-bordeaux-700 text-white py-4",
                            div { class: "max-w-6xl mx-auto px-6 flex flex-col sm:flex-row items-center justify-center gap-3 text-center",
                                span { class: "text-xl", "🛖" }
                                p { class: "font-body font-medium text-sm",
                                    "Prochain marché : "
                                    strong { "{date}" }
                                    " — {place}"
                                }
                            }
                        }
                    }
                } else { rsx! {} }
            } else { rsx! {} }
        }

        // ── Chiffres clés ─────────────────────────────────────────────────
        section { class: "py-16 bg-bordeaux-700 text-white",
            div { class: "max-w-5xl mx-auto px-6",
                div { class: "grid grid-cols-2 md:grid-cols-4 gap-8 text-center",
                    div {
                        p { class: "font-display text-4xl text-safran-400 mb-1", "100%" }
                        p { class: "font-body text-sm text-bordeaux-200", "Fait maison" }
                    }
                    div {
                        p { class: "font-display text-4xl text-safran-400 mb-1", "🦐" }
                        p { class: "font-body text-sm text-bordeaux-200", "Fruits de mer frais" }
                    }
                    div {
                        p { class: "font-display text-4xl text-safran-400 mb-1", "🥘" }
                        p { class: "font-body text-sm text-bordeaux-200", "Paella géante" }
                    }
                    div {
                        p { class: "font-display text-4xl text-safran-400 mb-1", "❤️" }
                        p { class: "font-body text-sm text-bordeaux-200", "Avec passion" }
                    }
                }
            }
        }

        // ── À propos ──────────────────────────────────────────────────────
        section { id: "about", class: "py-24 bg-creme-50",
            div { class: "max-w-6xl mx-auto px-6",
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-16 items-center",
                    div { class: "relative flex justify-center",
                        div { class: "relative",
                            img {
                                src: "/Logo.png",
                                alt: "La Poêlée du Bonheur",
                                class: "drop-shadow-xl",
                                style: "height: 300px; width: auto;"
                            }
                            div { class: "absolute -bottom-4 -right-4 bg-safran-500 text-white rounded-2xl py-3 px-5 shadow-lg",
                                p { class: "font-hand text-lg font-bold", "Devis gratuit !" }
                            }
                            div { class: "absolute -top-4 -left-4 bg-bordeaux-700 text-white rounded-2xl py-2 px-4 shadow-lg",
                                p { class: "font-hand text-base", "🦐 Fait maison" }
                            }
                        }
                    }

                    div {
                        span { class: "section-label text-safran-600 block mb-3", "Qui sommes-nous ?" }
                        h2 { class: "section-title text-ardoise-800 mb-6 leading-tight",
                            "Une cuisine "
                            span { class: "text-bordeaux-700", "généreuse" }
                            br {}
                            "et conviviale"
                        }
                        p { class: "font-body text-ardoise-600 leading-relaxed mb-4",
                            "La Poêlée du Bonheur, c'est la promesse d'une paella authentique et généreuse, préparée avec des produits frais de qualité. Nous nous déplaçons pour sublimer tous vos événements : mariages, anniversaires, fêtes de famille, séminaires..."
                        }
                        p { class: "font-body text-ardoise-600 leading-relaxed mb-8",
                            "Retrouvez-nous aussi sur les marchés locaux pour un avant-goût de bonheur !"
                        }
                        div { class: "flex flex-wrap gap-3",
                            span { class: "tag bg-bordeaux-100 text-bordeaux-700", "🥘 Paella géante" }
                            span { class: "tag bg-safran-100 text-safran-700", "🦐 Fruits de mer" }
                            span { class: "tag bg-creme-200 text-ardoise-700", "🫒 Méditerranéen" }
                            span { class: "tag bg-creme-200 text-ardoise-700", "🌿 Produits frais" }
                        }
                    }
                }
            }
        }

        // ── Menu ──────────────────────────────────────────────────────────
        section { id: "menu", class: "py-24 bg-white",
            div { class: "max-w-6xl mx-auto px-6",
                div { class: "text-center mb-14",
                    span { class: "section-label text-safran-600 block mb-3", "Notre carte" }
                    h2 { class: "section-title text-ardoise-800 mb-4",
                        "Des saveurs "
                        span { class: "text-bordeaux-700", "qui régalent" }
                    }
                    p { class: "font-body text-ardoise-500 max-w-xl mx-auto",
                        "Chaque plat est préparé le jour même avec des ingrédients frais et de saison."
                    }
                }

                {
                    let menu_ref = menu.read();
                    match menu_ref.as_ref() {
                        None => rsx! {
                            div { class: "flex justify-center py-16",
                                div { class: "w-12 h-12 border-4 border-bordeaux-700 border-t-transparent rounded-full animate-spin" }
                            }
                        },
                        Some(None) | Some(Some(..)) => {
                            let data_opt = menu_ref.as_ref().and_then(|o| o.as_ref());
                            if data_opt.map(|d| d.items.is_empty()).unwrap_or(true) {
                                rsx! {
                                    div { class: "text-center py-16",
                                        div { class: "text-6xl mb-4", "🥘" }
                                        p { class: "font-hand text-2xl text-ardoise-500", "La carte est en cours de préparation..." }
                                    }
                                }
                            } else {
                                let items = data_opt.unwrap().items.clone();
                                let categories = vec![MenuCategory::Starter, MenuCategory::MainDish, MenuCategory::Dessert];
                                rsx! {
                                    div { class: "space-y-16",
                                        for cat in categories {
                                            {
                                                let cat_items: Vec<_> = items.iter().filter(|i| i.category == cat).cloned().collect();
                                                if cat_items.is_empty() { rsx! {} } else {
                                                    let label = cat.label();
                                                    let emoji = cat.emoji();
                                                    rsx! {
                                                        div {
                                                            h3 { class: "font-hand text-3xl font-bold text-ardoise-800 mb-6 flex items-center gap-3",
                                                                span { class: "text-4xl", "{emoji}" }
                                                                "{label}"
                                                                span { class: "flex-1 h-px bg-creme-200 ml-2" }
                                                            }
                                                            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6",
                                                                for item in cat_items {
                                                                    div { key: "{item.id}", class: "card border border-creme-100",
                                                                        if let Some(url) = item.photo_url.clone() {
                                                                            img { src: "{url}", alt: "{item.name}", class: "w-full h-48 object-cover" }
                                                                        }
                                                                        div { class: "p-5",
                                                                            h4 { class: "font-hand text-xl font-bold text-ardoise-800 mb-1", "{item.name}" }
                                                                            p { class: "font-body text-ardoise-500 text-sm leading-relaxed mb-3", "{item.description}" }
                                                                            if let Some(p) = item.price_info {
                                                                                span { class: "tag bg-safran-100 text-safran-700", "{p}" }
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
                        }
                    }
                }

                div { class: "text-center mt-14",
                    Link { to: Route::Devis {}, class: "btn btn-primary text-base px-10 py-4", "🍽️ Composer mon menu sur mesure" }
                }
            }
        }

        // ── CTA final ─────────────────────────────────────────────────────
        section { class: "py-24 bg-gradient-to-br from-ardoise-800 to-ardoise-900 text-white text-center relative overflow-hidden",
            div {
                class: "absolute inset-0 opacity-10",
                style: "background-image: radial-gradient(circle, #e8a030 1px, transparent 1px); background-size: 50px 50px;"
            }
            div { class: "absolute top-0 left-1/2 -translate-x-1/2 w-96 h-96 bg-bordeaux-700/20 rounded-full blur-3xl" }

            div { class: "relative max-w-2xl mx-auto px-6",
                div { class: "text-6xl mb-6", "🥘" }
                h2 { class: "font-display text-4xl md:text-5xl mb-4",
                    "Votre événement,"
                    br {}
                    span { class: "text-safran-400", "notre bonheur !" }
                }
                p { class: "font-body text-ardoise-300 text-lg mb-10 leading-relaxed",
                    "Parlez-nous de votre projet. On vous prépare un devis gratuit et personnalisé."
                }
                div { class: "flex flex-col sm:flex-row gap-4 justify-center",
                    Link { to: Route::Devis {}, class: "btn btn-safran text-base px-10 py-4 shadow-xl", "Demander un devis gratuit" }
                    a { href: "tel:0745852654", class: "btn btn-white text-base px-8 py-4", "📞 07.45.85.26.54" }
                }
            }
        }
        Footer {}
        } // end flex col
    }
}
