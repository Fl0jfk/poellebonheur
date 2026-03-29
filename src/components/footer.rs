use leptos::*;
use leptos_router::A;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-ardoise-800 text-ardoise-300 pt-16 pb-6">
            <div class="max-w-6xl mx-auto px-6">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-12 mb-12">

                    // Branding
                    <div class="flex flex-col items-start gap-4">
                        <div class="flex items-center gap-4">
                            <img
                                src="/Logo.png"
                                alt="La Poêlée du Bonheur"
                                style="height: 90px; width: auto;"
                            />
                            <p class="font-display text-2xl text-white leading-tight">
                                "La Poêlée"<br/>
                                <span class="text-safran-400">"du Bonheur"</span>
                            </p>
                        </div>
                        <p class="text-sm leading-relaxed text-ardoise-400">
                            "Traiteur événementielle spécialisée en paella et cuisine méditerranéenne. "
                            "Fait maison, avec amour."
                        </p>
                        <div class="flex gap-2 flex-wrap">
                            <span class="tag bg-bordeaux-700/30 text-bordeaux-300">"🥘 Paella"</span>
                            <span class="tag bg-safran-500/20 text-safran-400">"🦐 Fruits de mer"</span>
                        </div>
                    </div>

                    // Navigation
                    <div>
                        <h4 class="text-safran-400 text-xs font-body font-bold uppercase tracking-widest mb-5">
                            "Navigation"
                        </h4>
                        <ul class="list-none m-0 p-0 flex flex-col gap-3">
                            <li>
                                <A href="/#about" class="text-sm text-ardoise-400 hover:text-safran-400 transition-colors font-body">
                                    "Notre histoire"
                                </A>
                            </li>
                            <li>
                                <A href="/#menu" class="text-sm text-ardoise-400 hover:text-safran-400 transition-colors font-body">
                                    "Nos plats"
                                </A>
                            </li>
                            <li>
                                <A href="/devis" class="text-sm text-ardoise-400 hover:text-safran-400 transition-colors font-body">
                                    "Demander un devis"
                                </A>
                            </li>
                        </ul>
                    </div>

                    // Contact
                    <div>
                        <h4 class="text-safran-400 text-xs font-body font-bold uppercase tracking-widest mb-5">
                            "Contact"
                        </h4>
                        <ul class="list-none m-0 p-0 flex flex-col gap-3">
                            <li>
                                <a
                                    href="tel:0745852654"
                                    class="text-sm text-ardoise-400 hover:text-safran-400 transition-colors font-body flex items-center gap-2"
                                >
                                    "📞 07.45.85.26.54"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="mailto:contact@lapoeleedubonheur.fr"
                                    class="text-sm text-ardoise-400 hover:text-safran-400 transition-colors font-body flex items-center gap-2"
                                >
                                    "✉️ contact@lapoeleedubonheur.fr"
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>

                <div class="border-t border-white/10 pt-6 text-center text-xs text-ardoise-600 font-body">
                    "© 2026 La Poêlée du Bonheur — Site réalisé avec ❤️ en Rust."
                </div>
            </div>
        </footer>
    }
}
