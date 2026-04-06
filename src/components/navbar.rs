use dioxus::prelude::*;
use crate::app::Route;

#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let open = menu_open();
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    if let Some(body) = doc.body() {
                        if open {
                            let _ = body.style().set_property("overflow", "hidden");
                        } else {
                            let _ = body.style().remove_property("overflow");
                        }
                    }
                    if let Some(html) = doc.document_element() {
                        let list = html.class_list();
                        if open {
                            let _ = list.add_1("overflow-hidden");
                        } else {
                            let _ = list.remove_1("overflow-hidden");
                        }
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = menu_open();
        }
    });

    rsx! {
        nav {
            class: "fixed top-0 left-0 right-0 z-50",
            input {
                r#type: "checkbox",
                id: "nav-toggle",
                checked: menu_open(),
                class: "peer pointer-events-none sr-only",
            }

            div {
                id: "nav-bar-shell",
                class: "\
                    relative z-20 w-full bg-white/70 backdrop-blur-2xl border-b border-white/40 \
                    shadow-[0_1px_12px_rgba(0,0,0,0.08)] pt-[env(safe-area-inset-top,0px)] \
                    peer-checked:[&_.bar-top]:translate-y-2 peer-checked:[&_.bar-top]:rotate-45 \
                    peer-checked:[&_.bar-mid]:opacity-0 peer-checked:[&_.bar-mid]:scale-x-0 \
                    peer-checked:[&_.bar-bot]:-translate-y-2 peer-checked:[&_.bar-bot]:-rotate-45 \
                ",

                div {
                    id: "nav-bar",
                    class: "max-w-6xl mx-auto px-4 flex items-center py-1.5 md:py-[10px]",

                    div { class: "w-full grid grid-cols-3 items-center md:flex md:justify-between md:gap-8",

                        div { class: "flex items-center",
                            Link {
                                to: Route::Home {},
                                class: "md:hidden no-underline overflow-visible",
                                onclick: move |_| menu_open.set(false),
                                img {
                                    src: "/Logo.png",
                                    alt: "La Poêlée du Bonheur",
                                    class: "\
                                        block h-[50px] w-[50px] origin-left scale-[1.28] object-contain \
                                    ",
                                }
                            }
                            Link {
                                to: Route::Home {},
                                class: "hidden md:flex items-center gap-3 no-underline group",
                                img {
                                    src: "/Logo.png",
                                    alt: "La Poêlée du Bonheur",
                                    class: "block h-[60px] w-[60px] group-hover:scale-105 transition-transform",
                                }
                                span {
                                    class: "font-display text-[1.8rem] leading-tight text-bordeaux-700",
                                    "La Poêlée du Bonheur"
                                }
                            }
                        }

                        div { class: "flex justify-center md:hidden",
                            span {
                                class: "\
                                    font-display text-[1.2rem] font-bold text-bordeaux-700 \
                                    leading-tight whitespace-nowrap \
                                ",
                                "La Poêlée du Bonheur"
                            }
                        }

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
                            div {
                                class: "\
                                    md:hidden flex flex-col justify-center items-center w-10 h-10 \
                                    cursor-pointer rounded-xl hover:bg-ardoise-100/60 transition-colors \
                                    select-none gap-[5px] \
                                ",
                                aria_label: "Menu",
                                onclick: move |_| menu_open.set(!menu_open()),
                                span { class: "bar-top block w-[22px] h-[2px] bg-ardoise-700 rounded-full transition-all duration-300 origin-center" }
                                span { class: "bar-mid block w-[22px] h-[2px] bg-ardoise-700 rounded-full transition-all duration-300" }
                                span { class: "bar-bot block w-[22px] h-[2px] bg-ardoise-700 rounded-full transition-all duration-300 origin-center" }
                            }
                        }
                    }
                }
            }

            div {
                id: "nav-mobile",
                class: "\
                    md:hidden fixed inset-0 z-10 box-border flex flex-col bg-white \
                    h-[100dvh] max-h-[100dvh] min-h-full \
                    pt-[calc(env(safe-area-inset-top,0px)+63px)] \
                    opacity-0 pointer-events-none -translate-y-2 \
                    transition-all duration-200 ease-out overflow-y-auto \
                    peer-checked:opacity-100 peer-checked:pointer-events-auto peer-checked:translate-y-0 \
                ",

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

                div { class: "shrink-0 px-6 py-6 border-t border-ardoise-100/40",
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
