use leptos::*;
use leptos_router::A;

#[component]
pub fn Navbar() -> impl IntoView {
    let (menu_open, set_menu_open) = create_signal(false);

    view! {
        <nav class="fixed top-0 left-0 right-0 z-50
                    bg-white/70 backdrop-blur-2xl
                    border-b border-white/40
                    shadow-[0_1px_12px_rgba(0,0,0,0.08)]">

            // Checkbox — contrôle le menu mobile (caché, piloté par le signal)
            <input
                type="checkbox"
                id="nav-toggle"
                prop:checked=menu_open
                on:change=move |e| set_menu_open.set(event_target_checked(&e))
            />

            // ── Barre principale ──────────────────────────────────────────
            // padding vertical identique au header du menu ouvert → hauteur cohérente
            <div id="nav-bar" class="max-w-6xl mx-auto px-4 flex items-center" style="padding-top: 10px; padding-bottom: 10px;">
                <div class="w-full grid grid-cols-3 items-center md:flex md:justify-between md:gap-8">

                    // Logo
                    <div class="flex items-center">
                        // Mobile
                        <A href="/" class="md:hidden no-underline">
                            <img
                                src="/Logo.png"
                                alt="La Poêlée du Bonheur"
                                style="height: 64px; width: 64px; display: block;"
                            />
                        </A>
                        // Desktop
                        <A href="/" class="hidden md:flex items-center gap-3 no-underline group">
                            <img
                                src="/Logo.png"
                                alt="La Poêlée du Bonheur"
                                style="height: 60px; width: 60px; display: block;"
                                class="group-hover:scale-105 transition-transform"
                            />
                            <span class="font-display text-bordeaux-700 leading-tight"
                                  style="font-size: 1.8rem; font-family: 'Amatic SC', cursive;">
                                "La Poêlée du Bonheur"
                            </span>
                        </A>
                    </div>

                    // Texte centré — mobile uniquement
                    <div class="flex justify-center md:hidden">
                        <span class="text-bordeaux-700 leading-tight whitespace-nowrap"
                              style="font-size: 1.35rem; font-family: 'Amatic SC', cursive; font-weight: 700;">
                            "La Poêlée du Bonheur"
                        </span>
                    </div>

                    // Liens desktop + bouton hamburger
                    <div class="flex items-center justify-end gap-6">
                        <ul class="hidden md:flex items-center gap-8 list-none m-0 p-0">
                            <li>
                                <A href="/#about" class="text-sm font-semibold font-body text-ardoise-700 hover:text-bordeaux-700 transition-colors">
                                    "Notre histoire"
                                </A>
                            </li>
                            <li>
                                <A href="/#menu" class="text-sm font-semibold font-body text-ardoise-700 hover:text-bordeaux-700 transition-colors">
                                    "Nos plats"
                                </A>
                            </li>
                            <li>
                                <A href="/devis" class="btn btn-safran text-sm px-5 py-2">
                                    "🍽️ Réserver"
                                </A>
                            </li>
                        </ul>
                        <label
                            for="nav-toggle"
                            class="md:hidden flex flex-col justify-center items-center w-10 h-10 cursor-pointer rounded-xl hover:bg-ardoise-100/60 transition-colors select-none gap-[5px]"
                            aria-label="Menu"
                        >
                            <span class="bar-top block w-[22px] h-[2px] bg-ardoise-700 rounded-full transition-all duration-300 origin-center"></span>
                            <span class="bar-mid block w-[22px] h-[2px] bg-ardoise-700 rounded-full transition-all duration-300"></span>
                            <span class="bar-bot block w-[22px] h-[2px] bg-ardoise-700 rounded-full transition-all duration-300 origin-center"></span>
                        </label>
                    </div>
                </div>
            </div>

            // ── Menu mobile — slide depuis sous la navbar (style Apple) ──────
            <div
                id="nav-mobile"
                class="bg-white flex flex-col border-t border-ardoise-100"
            >
                // Liens de navigation — on:click ferme le menu via le signal
                <div class="flex-1 flex flex-col justify-center px-6 py-8 gap-2">
                    <A href="/#about"
                        on:click=move |_| set_menu_open.set(false)
                        class="flex items-center gap-4 py-4 px-5 rounded-2xl font-body font-semibold text-ardoise-800 text-lg hover:bg-creme-100 transition-colors no-underline">
                        <span class="text-2xl">"🏡"</span>
                        "Notre histoire"
                    </A>
                    <A href="/#menu"
                        on:click=move |_| set_menu_open.set(false)
                        class="flex items-center gap-4 py-4 px-5 rounded-2xl font-body font-semibold text-ardoise-800 text-lg hover:bg-creme-100 transition-colors no-underline">
                        <span class="text-2xl">"🥘"</span>
                        "Nos plats"
                    </A>
                    <A href="/devis"
                        on:click=move |_| set_menu_open.set(false)
                        class="flex items-center gap-4 py-4 px-5 rounded-2xl font-body font-semibold text-ardoise-800 text-lg hover:bg-creme-100 transition-colors no-underline">
                        <span class="text-2xl">"🍽️"</span>
                        "Demander un devis"
                    </A>

                    <div class="my-2 border-t border-ardoise-100/60"></div>

                    <a href="https://www.facebook.com/people/La-Po%C3%AAl%C3%A9e-du-Bonheur/61572905885666/"
                        target="_blank" rel="noopener noreferrer"
                        on:click=move |_| set_menu_open.set(false)
                        class="flex items-center gap-4 py-4 px-5 rounded-2xl font-body font-semibold text-ardoise-800 text-lg hover:bg-blue-50 transition-colors no-underline">
                        <span class="text-2xl">"📘"</span>
                        "Facebook"
                    </a>
                    <a href="tel:0745852654"
                        on:click=move |_| set_menu_open.set(false)
                        class="flex items-center gap-4 py-4 px-5 rounded-2xl font-body font-semibold text-bordeaux-700 text-lg bg-bordeaux-50 hover:bg-bordeaux-100 transition-colors no-underline">
                        <span class="text-2xl">"📞"</span>
                        "07.45.85.26.54"
                    </a>
                </div>

                // Bouton CTA bas de page
                <div class="px-6 py-6 border-t border-ardoise-100/40" style="flex-shrink: 0;">
                    <A href="/devis"
                        on:click=move |_| set_menu_open.set(false)
                        class="btn btn-safran w-full justify-center text-base py-4">
                        "🍽️ Réserver un événement"
                    </A>
                </div>
            </div>
        </nav>
    }
}
