use dioxus::prelude::*;
use crate::app::Route;

#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

    // Scroll lock sur le body quand le menu mobile est ouvert
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            if menu_open() {
                body.style().set_property("overflow", "hidden").ok();
            } else {
                body.style().remove_property("overflow").ok();
            }
        }
    });

    rsx! {
        nav {
            class: "fixed top-0 left-0 right-0 z-50 bg-white/70 backdrop-blur-2xl border-b border-white/40 shadow-[0_1px_12px_rgba(0,0,0,0.08)]",

            // Checkbox cachée — pilotée par le signal (CSS anime le menu)
            input {
                r#type: "checkbox",
                id: "nav-toggle",
                checked: menu_open(),
                style: "display: none; position: absolute; pointer-events: none;",
            }

            // ── Barre principale ──────────────────────────────────────────
            div {
                id: "nav-bar",
                class: "max-w-6xl mx-auto px-4 flex items-center",
                style: "padding-top: 10px; padding-bottom: 10px;",

                div { class: "w-full grid grid-cols-3 items-center md:flex md:justify-between md:gap-8",

                    // Logo
                    div { class: "flex items-center",
                        // Mobile
                        Link {
                            to: Route::Home {},
                            class: "md:hidden no-underline",
                            onclick: move |_| menu_open.set(false),
                            img {
                                src: "/Logo.png",
                                alt: "La Poêlée du Bonheur",
                                style: "height: 64px; width: 64px; display: block;"
                            }
                        }
                        // Desktop
                        Link {
                            to: Route::Home {},
                            class: "hidden md:flex items-center gap-3 no-underline group",
                            img {
                                src: "/Logo.png",
                                alt: "La Poêlée du Bonheur",
                                style: "height: 60px; width: 60px; display: block;",
                                class: "group-hover:scale-105 transition-transform"
                            }
                            span {
                                class: "font-display text-bordeaux-700 leading-tight",
                                style: "font-size: 1.8rem; font-family: 'Amatic SC', cursive;",
                                "La Poêlée du Bonheur"
                            }
                        }
                    }

                    // Texte centré mobile
                    div { class: "flex justify-center md:hidden",
                        span {
                            class: "text-bordeaux-700 leading-tight whitespace-nowrap",
                            style: "font-size: 1.35rem; font-family: 'Amatic SC', cursive; font-weight: 700;",
                            "La Poêlée du Bonheur"
                        }
                    }

                    // Liens desktop + bouton hamburger
                    div { class: "flex items-center justify-end gap-6",
                        ul { class: "hidden md:flex items-center gap-8 list-none m-0 p-0",
                            li {
                                a {
                                    href: "/#about",
                                    class: "text-sm font-semibold font-body text-ardoise-700 hover:text-bordeaux-700 transition-colors",
                                    "Notre histoire"
                                }
                            }
                            li {
                                a {
                                    href: "/#menu",
                                    class: "text-sm font-semibold font-body text-ardoise-700 hover:text-bordeaux-700 transition-colors",
                                    "Nos plats"
                                }
                            }
                            li {
                                Link {
                                    to: Route::Devis {},
                                    class: "btn btn-safran text-sm px-5 py-2",
                                    "🍽️ Réserver"
                                }
                            }
                        }
                        // Hamburger — onclick toggle signal (pas de for= pour éviter double toggle)
                        div {
                            class: "md:hidden flex flex-col justify-center items-center w-10 h-10 cursor-pointer rounded-xl hover:bg-ardoise-100/60 transition-colors select-none gap-[5px]",
                            aria_label: "Menu",
                            onclick: move |_| menu_open.set(!menu_open()),
                            span { class: "bar-top block w-[22px] h-[2px] bg-ardoise-700 rounded-full transition-all duration-300 origin-center" }
                            span { class: "bar-mid block w-[22px] h-[2px] bg-ardoise-700 rounded-full transition-all duration-300" }
                            span { class: "bar-bot block w-[22px] h-[2px] bg-ardoise-700 rounded-full transition-all duration-300 origin-center" }
                        }
                    }
                }
            }

            // ── Menu mobile ───────────────────────────────────────────────
            div {
                id: "nav-mobile",
                class: "bg-white flex flex-col border-t border-ardoise-100",

                div { class: "flex-1 flex flex-col justify-center px-6 py-8 gap-2",
                    a {
                        href: "/#about",
                        onclick: move |_| menu_open.set(false),
                        class: "flex items-center gap-4 py-4 px-5 rounded-2xl font-body font-semibold text-ardoise-800 text-lg hover:bg-creme-100 transition-colors no-underline",
                        span { class: "text-2xl", "🏡" }
                        "Notre histoire"
                    }
                    a {
                        href: "/#menu",
                        onclick: move |_| menu_open.set(false),
                        class: "flex items-center gap-4 py-4 px-5 rounded-2xl font-body font-semibold text-ardoise-800 text-lg hover:bg-creme-100 transition-colors no-underline",
                        span { class: "text-2xl", "🥘" }
                        "Nos plats"
                    }
                    Link {
                        to: Route::Devis {},
                        onclick: move |_| menu_open.set(false),
                        class: "flex items-center gap-4 py-4 px-5 rounded-2xl font-body font-semibold text-ardoise-800 text-lg hover:bg-creme-100 transition-colors no-underline",
                        span { class: "text-2xl", "🍽️" }
                        "Demander un devis"
                    }

                    div { class: "my-2 border-t border-ardoise-100/60" }

                    a {
                        href: "https://www.facebook.com/people/La-Po%C3%AAl%C3%A9e-du-Bonheur/61572905885666/",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        onclick: move |_| menu_open.set(false),
                        class: "flex items-center gap-4 py-4 px-5 rounded-2xl font-body font-semibold text-ardoise-800 text-lg hover:bg-blue-50 transition-colors no-underline",
                        span { class: "text-2xl", "📘" }
                        "Facebook"
                    }
                    a {
                        href: "tel:0745852654",
                        onclick: move |_| menu_open.set(false),
                        class: "flex items-center gap-4 py-4 px-5 rounded-2xl font-body font-semibold text-bordeaux-700 text-lg bg-bordeaux-50 hover:bg-bordeaux-100 transition-colors no-underline",
                        span { class: "text-2xl", "📞" }
                        "07.45.85.26.54"
                    }
                }

                div { class: "px-6 py-6 border-t border-ardoise-100/40", style: "flex-shrink: 0;",
                    Link {
                        to: Route::Devis {},
                        onclick: move |_| menu_open.set(false),
                        class: "btn btn-safran w-full justify-center text-base py-4",
                        "🍽️ Réserver un événement"
                    }
                }
            }
        }
    }
}
