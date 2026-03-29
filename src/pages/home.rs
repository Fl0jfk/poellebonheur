use leptos::*;
use leptos_router::A;

use crate::components::{footer::Footer, navbar::Navbar};
use crate::models::{MenuCategory, MenuItem};
use crate::server::functions::{get_market, get_menu};

#[component]
fn PhotoCollage() -> impl IntoView {
    view! {
        <div class="collage-wrapper">
            <input type="radio" name="collage" id="cp0" checked />
            <input type="radio" name="collage" id="cp1" />
            <input type="radio" name="collage" id="cp2" />
            <input type="radio" name="collage" id="cp3" />
            <input type="radio" name="collage" id="cp4" />

            <label for="cp0" class="collage-photo collage-p0">
                <img src="/Plat1.jpg" alt="Paella géante"
                     style="width:100%;height:100%;object-fit:cover;display:block;pointer-events:none;"/>
            </label>
            <label for="cp1" class="collage-photo collage-p1">
                <img src="/Entree1.jpg" alt="Fruits de mer"
                     style="width:100%;height:100%;object-fit:cover;display:block;pointer-events:none;"/>
            </label>
            <label for="cp2" class="collage-photo collage-p2">
                <img src="/Dessert1.jpg" alt="Desserts maison"
                     style="width:100%;height:100%;object-fit:cover;display:block;pointer-events:none;"/>
            </label>
            <label for="cp3" class="collage-photo collage-p3">
                <img src="/Repas%202.jpg" alt="Repas convivial"
                     style="width:100%;height:100%;object-fit:cover;display:block;pointer-events:none;"/>
            </label>
            <label for="cp4" class="collage-photo collage-p4">
                <img src="/Repas%203.jpg" alt="Repas festif"
                     style="width:100%;height:100%;object-fit:cover;display:block;pointer-events:none;"/>
            </label>
        </div>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let market = create_resource(|| (), |_| get_market());
    let menu   = create_resource(|| (), |_| get_menu());

    view! {
        <div class="min-h-screen flex flex-col">
            <Navbar />

            // ── Hero ─────────────────────────────────────────────────────────
            <section class="relative min-h-screen flex items-center justify-center pt-16">

                // Fond + décorations dans un calque clippé séparément
                <div class="absolute inset-0 overflow-hidden">
                    <div class="absolute inset-0 bg-gradient-to-br from-ardoise-700 via-ardoise-800 to-ardoise-900"></div>
                    <div class="absolute inset-0 opacity-5"
                         style="background-image: radial-gradient(circle, #e8a030 1px, transparent 1px); background-size: 40px 40px;">
                    </div>
                    <div class="absolute top-20 left-10 w-72 h-72 bg-bordeaux-700/20 rounded-full blur-3xl"></div>
                    <div class="absolute bottom-20 right-10 w-96 h-96 bg-safran-500/15 rounded-full blur-3xl"></div>
                </div>

                <div class="relative z-10 text-center w-full max-w-4xl mx-auto">

                    // Collage — wrapper avec marge bottom responsive
                    <div class="flex justify-center mb-8 sm:mb-12">
                        <PhotoCollage />
                    </div>

                    <div class="px-6">
                    <h1 class="font-display text-5xl md:text-7xl text-white mb-4 drop-shadow-lg">
                        "La Poêlée"
                        <br/>
                        <span class="text-safran-400">"du Bonheur"</span>
                    </h1>

                    <p class="font-hand text-2xl text-ardoise-300 mb-3">"Traiteur événementielle"</p>

                    <p class="font-body text-ardoise-300 text-lg max-w-2xl mx-auto mb-10 leading-relaxed">
                        "Paella géante, fruits de mer et saveurs méditerranéennes cuisinés avec passion "
                        "pour vos mariages, anniversaires et marchés. 🦐🥘"
                    </p>

                    <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
                        <A href="/devis" class="btn btn-safran text-base px-10 py-4 shadow-lg">
                            "🍽️ Demander un devis"
                        </A>
                        <a href="#menu" class="btn btn-ghost text-base px-8 py-4">
                            "Voir notre carte"
                        </a>
                    </div>
                    </div> // fin px-6
                </div>
                <div class="absolute bottom-8 xl:bottom-4 left-1/2 -translate-x-1/2 animate-bounce">
                    <svg class="w-6 h-6 text-white/40" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                    </svg>
                </div>
            </section>

            // ── Marché ───────────────────────────────────────────────────────
            <Suspense fallback=|| ()>
                {move || market.get().and_then(|r: Result<_, _>| r.ok()).and_then(|m| {
                    if m.active {
                        Some(view! {
                            <div class="bg-bordeaux-700 text-white py-4">
                                <div class="max-w-6xl mx-auto px-6 flex flex-col sm:flex-row items-center justify-center gap-3 text-center">
                                    <span class="text-xl">"🛖"</span>
                                    <p class="font-body font-medium text-sm">
                                        "Prochain marché : "
                                        <strong>{m.date.unwrap_or_default()}</strong>
                                        " — "
                                        {m.place.unwrap_or_default()}
                                    </p>
                                </div>
                            </div>
                        })
                    } else {
                        None
                    }
                })}
            </Suspense>

            // ── Chiffres clés ────────────────────────────────────────────────
            <section class="py-16 bg-bordeaux-700 text-white">
                <div class="max-w-5xl mx-auto px-6">
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-8 text-center">
                        <div>
                            <p class="font-display text-4xl text-safran-400 mb-1">"100%"</p>
                            <p class="font-body text-sm text-bordeaux-200">"Fait maison"</p>
                        </div>
                        <div>
                            <p class="font-display text-4xl text-safran-400 mb-1">"🦐"</p>
                            <p class="font-body text-sm text-bordeaux-200">"Fruits de mer frais"</p>
                        </div>
                        <div>
                            <p class="font-display text-4xl text-safran-400 mb-1">"🥘"</p>
                            <p class="font-body text-sm text-bordeaux-200">"Paella géante"</p>
                        </div>
                        <div>
                            <p class="font-display text-4xl text-safran-400 mb-1">"❤️"</p>
                            <p class="font-body text-sm text-bordeaux-200">"Avec passion"</p>
                        </div>
                    </div>
                </div>
            </section>

            // ── À propos ─────────────────────────────────────────────────────
            <section id="about" class="py-24 bg-creme-50">
                <div class="max-w-6xl mx-auto px-6">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-16 items-center">

                        // Illustration / logo grand
                        <div class="relative flex justify-center">
                            <div class="relative">
                                <img
                                    src="/Logo.png"
                                    alt="La Poêlée du Bonheur"
                                    class="drop-shadow-xl"
                                    style="height: 300px; width: auto;"
                                />
                                <div class="absolute -bottom-4 -right-4 bg-safran-500 text-white rounded-2xl py-3 px-5 shadow-lg">
                                    <p class="font-hand text-lg font-bold">"Devis gratuit !"</p>
                                </div>
                                <div class="absolute -top-4 -left-4 bg-bordeaux-700 text-white rounded-2xl py-2 px-4 shadow-lg">
                                    <p class="font-hand text-base">"🦐 Fait maison"</p>
                                </div>
                            </div>
                        </div>

                        <div>
                            <span class="section-label text-safran-600 block mb-3">"Qui sommes-nous ?"</span>
                            <h2 class="section-title text-ardoise-800 mb-6 leading-tight">
                                "Une cuisine "<span class="text-bordeaux-700">"généreuse"</span>
                                <br/>"et conviviale"
                            </h2>
                            <p class="font-body text-ardoise-600 leading-relaxed mb-4">
                                "La Poêlée du Bonheur, c'est la promesse d'une paella authentique et généreuse, "
                                "préparée avec des produits frais de qualité. Nous nous déplaçons pour sublimer "
                                "tous vos événements : mariages, anniversaires, fêtes de famille, séminaires..."
                            </p>
                            <p class="font-body text-ardoise-600 leading-relaxed mb-8">
                                "Retrouvez-nous aussi sur les marchés locaux pour un avant-goût de bonheur !"
                            </p>
                            <div class="flex flex-wrap gap-3">
                                <span class="tag bg-bordeaux-100 text-bordeaux-700">"🥘 Paella géante"</span>
                                <span class="tag bg-safran-100 text-safran-700">"🦐 Fruits de mer"</span>
                                <span class="tag bg-creme-200 text-ardoise-700">"🫒 Méditerranéen"</span>
                                <span class="tag bg-creme-200 text-ardoise-700">"🌿 Produits frais"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // ── Menu ─────────────────────────────────────────────────────────
            <section id="menu" class="py-24 bg-white">
                <div class="max-w-6xl mx-auto px-6">
                    <div class="text-center mb-14">
                        <span class="section-label text-safran-600 block mb-3">"Notre carte"</span>
                        <h2 class="section-title text-ardoise-800 mb-4">
                            "Des saveurs "<span class="text-bordeaux-700">"qui régalent"</span>
                        </h2>
                        <p class="font-body text-ardoise-500 max-w-xl mx-auto">
                            "Chaque plat est préparé le jour même avec des ingrédients frais et de saison."
                        </p>
                    </div>

                    <Suspense fallback=move || view! {
                        <div class="flex justify-center py-16">
                            <div class="w-12 h-12 border-4 border-bordeaux-700 border-t-transparent rounded-full animate-spin"></div>
                        </div>
                    }>
                        {move || menu.get().map(|result| match result {
                            Err(_) => view! {
                                <p class="text-center text-ardoise-400 py-12 font-body">
                                    "Impossible de charger la carte pour le moment."
                                </p>
                            }.into_view(),
                            Ok(data) if data.items.is_empty() => view! {
                                <div class="text-center py-16">
                                    <div class="text-6xl mb-4">"🥘"</div>
                                    <p class="font-hand text-2xl text-ardoise-500">
                                        "La carte est en cours de préparation..."
                                    </p>
                                </div>
                            }.into_view(),
                            Ok(data) => {
                                let categories = vec![
                                    MenuCategory::Starter,
                                    MenuCategory::MainDish,
                                    MenuCategory::Dessert,
                                ];
                                view! {
                                    <div class="space-y-16">
                                        {categories.into_iter().map(|cat| {
                                            let items: Vec<MenuItem> = data.items.iter()
                                                .filter(|i| i.category == cat)
                                                .cloned()
                                                .collect();
                                            if items.is_empty() {
                                                return view! { <div></div> }.into_view();
                                            }
                                            let label = cat.label();
                                            let emoji = cat.emoji();
                                            view! {
                                                <div>
                                                    <h3 class="font-hand text-3xl font-bold text-ardoise-800 mb-6 flex items-center gap-3">
                                                        <span class="text-4xl">{emoji}</span>
                                                        {label}
                                                        <span class="flex-1 h-px bg-creme-200 ml-2"></span>
                                                    </h3>
                                                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                                                        {items.into_iter().map(|item| view! {
                                                            <div class="card border border-creme-100">
                                                                {item.photo_url.as_ref().map(|url: &String| view! {
                                                                    <img
                                                                        src=url.clone()
                                                                        alt=item.name.clone()
                                                                        class="w-full h-48 object-cover"
                                                                    />
                                                                })}
                                                                <div class="p-5">
                                                                    <h4 class="font-hand text-xl font-bold text-ardoise-800 mb-1">
                                                                        {item.name}
                                                                    </h4>
                                                                    <p class="font-body text-ardoise-500 text-sm leading-relaxed mb-3">
                                                                        {item.description}
                                                                    </p>
                                                                    {item.price_info.map(|p| view! {
                                                                        <span class="tag bg-safran-100 text-safran-700">{p}</span>
                                                                    })}
                                                                </div>
                                                            </div>
                                                        }).collect_view()}
                                                    </div>
                                                </div>
                                            }.into_view()
                                        }).collect_view()}
                                    </div>
                                }.into_view()
                            }
                        })}
                    </Suspense>

                    <div class="text-center mt-14">
                        <A href="/devis" class="btn btn-primary text-base px-10 py-4">
                            "🍽️ Composer mon menu sur mesure"
                        </A>
                    </div>
                </div>
            </section>

            // ── CTA final ────────────────────────────────────────────────────
            <section class="py-24 bg-gradient-to-br from-ardoise-800 to-ardoise-900 text-white text-center relative overflow-hidden">
                <div class="absolute inset-0 opacity-10"
                     style="background-image: radial-gradient(circle, #e8a030 1px, transparent 1px); background-size: 50px 50px;">
                </div>
                <div class="absolute top-0 left-1/2 -translate-x-1/2 w-96 h-96 bg-bordeaux-700/20 rounded-full blur-3xl"></div>

                <div class="relative max-w-2xl mx-auto px-6">
                    <div class="text-6xl mb-6">"🥘"</div>
                    <h2 class="font-display text-4xl md:text-5xl mb-4">
                        "Votre événement,"<br/>
                        <span class="text-safran-400">"notre bonheur !"</span>
                    </h2>
                    <p class="font-body text-ardoise-300 text-lg mb-10 leading-relaxed">
                        "Parlez-nous de votre projet. On vous prépare un devis gratuit et personnalisé."
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <A href="/devis" class="btn btn-safran text-base px-10 py-4 shadow-xl">
                            "Demander un devis gratuit"
                        </A>
                        <a href="tel:0745852654" class="btn btn-white text-base px-8 py-4">
                            "📞 07.45.85.26.54"
                        </a>
                    </div>
                </div>
            </section>

            <Footer />
        </div>
    }
}
