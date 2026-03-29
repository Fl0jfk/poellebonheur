use leptos::*;
use leptos_router::use_navigate;

use crate::models::{CreateMenuItemPayload, MarketInfo, MenuCategory, QuoteStatus};
use crate::server::functions::*;

// ── Page principale ───────────────────────────────────────────────────────────

#[component]
pub fn AdminPage() -> impl IntoView {
    let auth = create_resource(|| (), |_| check_admin_auth());

    view! {
        <div class="min-h-screen bg-cream-50">
            <Suspense fallback=move || view! {
                <div class="min-h-screen flex items-center justify-center">
                    <div class="w-10 h-10 border-4 border-primary-600 border-t-transparent rounded-full animate-spin"></div>
                </div>
            }>
                {move || auth.get().map(|result: Result<bool, _>| {
                    let is_auth = result.unwrap_or(false);
                    if is_auth {
                        view! { <AdminDashboard/> }.into_view()
                    } else {
                        view! { <LoginForm on_success=move || auth.refetch() /> }.into_view()
                    }
                })}
            </Suspense>
        </div>
    }
}

// ── Formulaire de connexion ───────────────────────────────────────────────────

#[component]
fn LoginForm(on_success: impl Fn() + 'static) -> impl IntoView {
    let (password, set_password) = create_signal(String::new());
    let (loading,  set_loading)  = create_signal(false);
    let (error,    set_error)    = create_signal::<Option<String>>(None);

    let on_success = std::rc::Rc::new(on_success);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if loading.get() { return; }
        set_loading.set(true);
        set_error.set(None);
        let pwd = password.get();
        let on_success = on_success.clone();
        spawn_local(async move {
            match admin_login(pwd).await {
                Ok(_)  => on_success(),
                Err(e) => set_error.set(Some(format!("{e}"))),
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center px-4">
            <div class="bg-white rounded-3xl shadow-xl p-10 w-full max-w-sm">
                <div class="text-center mb-8">
                    <div class="text-5xl mb-4">"🔐"</div>
                    <h1 class="font-display text-2xl text-cream-900">"Espace admin"</h1>
                    <p class="text-cream-600 text-sm mt-1">"Nusha Traiteur"</p>
                </div>
                <form on:submit=on_submit class="space-y-5">
                    <div>
                        <label class="form-label">"Mot de passe"</label>
                        <input
                            type="password"
                            class="form-input"
                            placeholder="••••••••"
                            required
                            autofocus
                            prop:value=password
                            on:input=move |e| set_password.set(event_target_value(&e))
                        />
                    </div>
                    {move || error.get().map(|msg| view! {
                        <p class="text-red-600 text-sm">{msg}</p>
                    })}
                    <button
                        type="submit"
                        class="btn btn-primary w-full justify-center py-3"
                        disabled=move || loading.get()
                    >
                        {move || if loading.get() { "Connexion..." } else { "Se connecter" }}
                    </button>
                </form>
            </div>
        </div>
    }
}

// ── Dashboard ─────────────────────────────────────────────────────────────────

#[component]
fn AdminDashboard() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("quotes");

    let logout_action = create_action(|_: &()| admin_logout());
    let navigate = use_navigate();
    let _ = create_effect(move |_| {
        if logout_action.value().get().and_then(|r: Result<(), _>| r.ok()).is_some() {
            navigate("/admin", Default::default());
        }
    });

    view! {
        <div>
            // Topbar
            <header class="bg-white border-b border-cream-200 sticky top-0 z-30">
                <div class="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
                    <h1 class="font-display text-xl text-cream-900">"Admin — Nusha Traiteur"</h1>
                    <button
                        class="btn btn-outline text-sm px-4 py-2"
                        on:click=move |_| logout_action.dispatch(())
                    >
                        "Déconnexion"
                    </button>
                </div>
            </header>

            // Tabs
            <div class="max-w-6xl mx-auto px-6 pt-8">
                <div class="flex gap-2 mb-8 border-b border-cream-200">
                    <TabButton label="Devis" id="quotes" active=active_tab set_active=set_active_tab />
                    <TabButton label="Menu" id="menu" active=active_tab set_active=set_active_tab />
                    <TabButton label="Marché" id="market" active=active_tab set_active=set_active_tab />
                </div>

                {move || match active_tab.get() {
                    "quotes" => view! { <QuotesPanel/> }.into_view(),
                    "menu"   => view! { <MenuPanel/> }.into_view(),
                    "market" => view! { <MarketPanel/> }.into_view(),
                    _        => view! { <div></div> }.into_view(),
                }}
            </div>
        </div>
    }
}

#[component]
fn TabButton(
    label: &'static str,
    id: &'static str,
    active: ReadSignal<&'static str>,
    set_active: WriteSignal<&'static str>,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                if active.get() == id {
                    "px-4 py-2.5 text-sm font-semibold text-primary-600 border-b-2 border-primary-600 -mb-px"
                } else {
                    "px-4 py-2.5 text-sm font-medium text-cream-600 hover:text-primary-600 border-b-2 border-transparent -mb-px"
                }
            }
            on:click=move |_| set_active.set(id)
        >
            {label}
        </button>
    }
}

// ── Onglet Devis ─────────────────────────────────────────────────────────────

#[component]
fn QuotesPanel() -> impl IntoView {
    let quotes = create_resource(|| (), |_| get_quotes());

    view! {
        <div class="pb-16">
            <div class="flex items-center justify-between mb-6">
                <h2 class="font-display text-2xl text-cream-900">"Demandes de devis"</h2>
                <button
                    class="btn btn-outline text-sm px-4 py-2"
                    on:click=move |_| quotes.refetch()
                >
                    "Actualiser"
                </button>
            </div>

            <Suspense fallback=move || view! {
                <div class="flex justify-center py-12">
                    <div class="w-8 h-8 border-4 border-primary-600 border-t-transparent rounded-full animate-spin"></div>
                </div>
            }>
                {move || quotes.get().map(|result: Result<crate::models::QuotesData, leptos::ServerFnError>| match result {
                    Err(e) => view! {
                        <p class="text-red-600 py-4">"Erreur : "{format!("{e}")}</p>
                    }.into_view(),
                    Ok(data) if data.quotes.is_empty() => view! {
                        <div class="text-center py-16 text-cream-500">
                            <div class="text-5xl mb-4">"📭"</div>
                            <p>"Aucune demande de devis pour le moment."</p>
                        </div>
                    }.into_view(),
                    Ok(data) => {
                        let mut quotes_sorted = data.quotes.clone();
                        quotes_sorted.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                        view! {
                            <div class="space-y-4">
                                {quotes_sorted.into_iter().map(|q| {
                                    let status_cls = match q.status {
                                        QuoteStatus::Pending  => "tag bg-yellow-100 text-yellow-700",
                                        QuoteStatus::Viewed   => "tag bg-blue-100 text-blue-700",
                                        QuoteStatus::Replied  => "tag bg-green-100 text-green-700",
                                    };
                                    let status_label = match q.status {
                                        QuoteStatus::Pending  => "En attente",
                                        QuoteStatus::Viewed   => "Vue",
                                        QuoteStatus::Replied  => "Répondue",
                                    };
                                    view! {
                                        <div class="bg-white rounded-2xl shadow-sm border border-cream-200 p-6">
                                            <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3 mb-4">
                                                <div>
                                                    <h3 class="font-semibold text-cream-900 text-lg">
                                                        {format!("{} {}", q.first_name, q.last_name)}
                                                    </h3>
                                                    <p class="text-cream-500 text-sm">{q.created_at.chars().take(10).collect::<String>()}</p>
                                                </div>
                                                <span class=status_cls>{status_label}</span>
                                            </div>
                                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-sm mb-4">
                                                <div>
                                                    <span class="text-cream-500">"📧 "</span>
                                                    <a href=format!("mailto:{}", q.email) class="text-primary-600 hover:underline">{q.email}</a>
                                                </div>
                                                {if !q.phone.is_empty() { view! {
                                                    <div>
                                                        <span class="text-cream-500">"📞 "</span>
                                                        <a href=format!("tel:{}", q.phone) class="text-primary-600 hover:underline">{q.phone}</a>
                                                    </div>
                                                }.into_view()} else { view! { <div></div> }.into_view() }}
                                                <div>
                                                    <span class="text-cream-500">"📅 "</span>
                                                    {if q.event_date.is_empty() { "Date non précisée".to_string() } else { q.event_date }}
                                                </div>
                                                <div>
                                                    <span class="text-cream-500">"👥 "</span>
                                                    {format!("{} personnes", q.number_of_people)}
                                                </div>
                                                {if !q.event_place.is_empty() { view! {
                                                    <div class="sm:col-span-2">
                                                        <span class="text-cream-500">"📍 "</span>
                                                        {q.event_place}
                                                    </div>
                                                }.into_view()} else { view! {<div></div>}.into_view() }}
                                            </div>
                                            <div class="border-t border-cream-100 pt-3 text-sm space-y-1.5">
                                                {if !q.starters.is_empty() { view! {
                                                    <p><span class="text-cream-500 font-medium">"Entrées : "</span>{q.starters.join(", ")}</p>
                                                }.into_view()} else { view! {<span></span>}.into_view() }}
                                                {if !q.main_dish.is_empty() { view! {
                                                    <p><span class="text-cream-500 font-medium">"Plat : "</span>{q.main_dish}</p>
                                                }.into_view()} else { view! {<span></span>}.into_view() }}
                                                {if !q.desserts.is_empty() { view! {
                                                    <p><span class="text-cream-500 font-medium">"Desserts : "</span>{q.desserts.join(", ")}</p>
                                                }.into_view()} else { view! {<span></span>}.into_view() }}
                                                {q.message.map(|m| view! {
                                                    <p class="text-cream-600 italic mt-2">"\""  {m} "\""</p>
                                                })}
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_view()
                    }
                })}
            </Suspense>
        </div>
    }
}

// ── Onglet Menu ───────────────────────────────────────────────────────────────

#[component]
fn MenuPanel() -> impl IntoView {
    let menu = create_resource(|| (), |_| get_menu());

    let (name,        set_name)        = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (category,    set_category)    = create_signal("starter".to_string());
    let (price_info,  set_price_info)  = create_signal(String::new());
    let (form_error,  set_form_error)  = create_signal::<Option<String>>(None);
    let (saving,      set_saving)      = create_signal(false);

    let on_add = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if saving.get() { return; }
        let cat = match category.get().as_str() {
            "main_dish" => MenuCategory::MainDish,
            "dessert"   => MenuCategory::Dessert,
            _           => MenuCategory::Starter,
        };
        let payload = CreateMenuItemPayload {
            name:        name.get(),
            description: description.get(),
            category:    cat,
            price_info:  if price_info.get().is_empty() { None } else { Some(price_info.get()) },
        };
        set_saving.set(true);
        set_form_error.set(None);
        spawn_local(async move {
            match create_menu_item(payload).await {
                Ok(_) => {
                    set_name.set(String::new());
                    set_description.set(String::new());
                    set_price_info.set(String::new());
                    menu.refetch();
                }
                Err(e) => set_form_error.set(Some(format!("{e}"))),
            }
            set_saving.set(false);
        });
    };

    view! {
        <div class="pb-16 space-y-10">
            // Formulaire d'ajout
            <div class="bg-white rounded-2xl shadow-sm border border-cream-200 p-6">
                <h2 class="font-display text-xl text-cream-900 mb-5">"Ajouter un plat"</h2>
                <form on:submit=on_add class="space-y-4">
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                        <div>
                            <label class="form-label">"Nom *"</label>
                            <input type="text" class="form-input" required
                                prop:value=name
                                on:input=move |e| set_name.set(event_target_value(&e))
                            />
                        </div>
                        <div>
                            <label class="form-label">"Catégorie"</label>
                            <select class="form-input"
                                on:change=move |e| set_category.set(event_target_value(&e))
                            >
                                <option value="starter">"Entrée"</option>
                                <option value="main_dish">"Plat principal"</option>
                                <option value="dessert">"Dessert"</option>
                            </select>
                        </div>
                    </div>
                    <div>
                        <label class="form-label">"Description"</label>
                        <input type="text" class="form-input"
                            prop:value=description
                            on:input=move |e| set_description.set(event_target_value(&e))
                        />
                    </div>
                    <div>
                        <label class="form-label">"Info prix (optionnel)"</label>
                        <input type="text" class="form-input" placeholder="ex: 8€/pers."
                            prop:value=price_info
                            on:input=move |e| set_price_info.set(event_target_value(&e))
                        />
                    </div>
                    {move || form_error.get().map(|msg| view! {
                        <p class="text-red-600 text-sm">{msg}</p>
                    })}
                    <button type="submit" class="btn btn-primary px-6" disabled=move || saving.get()>
                        {move || if saving.get() { "Ajout..." } else { "Ajouter le plat" }}
                    </button>
                </form>
            </div>

            // Liste des plats
            <div>
                <h2 class="font-display text-xl text-cream-900 mb-5">"Carte actuelle"</h2>
                <Suspense fallback=move || view! {
                    <div class="flex justify-center py-8">
                        <div class="w-8 h-8 border-4 border-primary-600 border-t-transparent rounded-full animate-spin"></div>
                    </div>
                }>
                    {move || menu.get().map(|result: Result<crate::models::MenuData, leptos::ServerFnError>| match result {
                        Err(e) => view! { <p class="text-red-600 text-sm">{format!("{e}")}</p> }.into_view(),
                        Ok(data) if data.items.is_empty() => view! {
                            <p class="text-cream-500 text-sm">"Aucun plat dans la carte."</p>
                        }.into_view(),
                        Ok(data) => view! {
                            <div class="space-y-2">
                                {data.items.into_iter().map(|item| {
                                    let item_id = item.id.clone();
                                    let delete_action = create_action(move |id: &String| {
                                        let id = id.clone();
                                        async move { delete_menu_item(id).await }
                                    });
                                    let _ = create_effect(move |_| {
                                        if delete_action.value().get().and_then(|r: Result<(), _>| r.ok()).is_some() {
                                            menu.refetch();
                                        }
                                    });
                                    view! {
                                        <div class="bg-white rounded-xl border border-cream-200 px-5 py-4 flex items-center justify-between gap-4">
                                            <div class="flex-1 min-w-0">
                                                <div class="flex items-center gap-2 mb-0.5">
                                                    <span class="font-medium text-cream-900">{item.name}</span>
                                                    <span class="tag bg-cream-100 text-cream-600 text-xs">{item.category.label()}</span>
                                                    {item.price_info.map(|p| view! {
                                                        <span class="tag bg-primary-100 text-primary-700 text-xs">{p}</span>
                                                    })}
                                                </div>
                                                <p class="text-cream-500 text-sm truncate">{item.description}</p>
                                            </div>
                                            <button
                                                class="text-red-400 hover:text-red-600 transition-colors p-1.5 rounded-lg hover:bg-red-50 flex-shrink-0"
                                                on:click=move |_| delete_action.dispatch(item_id.clone())
                                                title="Supprimer"
                                            >
                                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                                </svg>
                                            </button>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_view(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}

// ── Onglet Marché ─────────────────────────────────────────────────────────────

#[component]
fn MarketPanel() -> impl IntoView {
    let market = create_resource(|| (), |_| get_market());

    let (date,    set_date)    = create_signal(String::new());
    let (place,   set_place)   = create_signal(String::new());
    let (active,  set_active)  = create_signal(false);
    let (saving,  set_saving)  = create_signal(false);
    let (saved,   set_saved)   = create_signal(false);
    let (err_msg, set_err_msg) = create_signal::<Option<String>>(None);

    let _ = create_effect(move |_| {
        if let Some(Ok(m)) = market.get() {
            set_date.set(m.date.unwrap_or_default());
            set_place.set(m.place.unwrap_or_default());
            set_active.set(m.active);
        }
    });

    let on_save = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if saving.get() { return; }
        set_saving.set(true);
        set_err_msg.set(None);
        set_saved.set(false);
        let info = MarketInfo {
            date:   if date.get().is_empty() { None } else { Some(date.get()) },
            place:  if place.get().is_empty() { None } else { Some(place.get()) },
            active: active.get(),
        };
        spawn_local(async move {
            match update_market(info).await {
                Ok(_) => {
                    set_saved.set(true);
                    market.refetch();
                }
                Err(e) => set_err_msg.set(Some(format!("{e}"))),
            }
            set_saving.set(false);
        });
    };

    view! {
        <div class="pb-16 max-w-lg">
            <h2 class="font-display text-2xl text-cream-900 mb-6">"Prochain marché"</h2>
            <div class="bg-white rounded-2xl shadow-sm border border-cream-200 p-6">
                <form on:submit=on_save class="space-y-5">
                    <div>
                        <label class="form-label">"Date"</label>
                        <input type="text" class="form-input" placeholder="ex: Samedi 5 avril 2025"
                            prop:value=date
                            on:input=move |e| set_date.set(event_target_value(&e))
                        />
                    </div>
                    <div>
                        <label class="form-label">"Lieu"</label>
                        <input type="text" class="form-input" placeholder="ex: Marché de Montrouge"
                            prop:value=place
                            on:input=move |e| set_place.set(event_target_value(&e))
                        />
                    </div>
                    <label class="flex items-center gap-3 cursor-pointer">
                        <input
                            type="checkbox"
                            class="w-5 h-5 rounded accent-primary-600"
                            prop:checked=active
                            on:change=move |e| set_active.set(event_target_checked(&e))
                        />
                        <span class="text-sm font-medium text-cream-800">"Afficher sur la page d'accueil"</span>
                    </label>
                    {move || err_msg.get().map(|msg| view! {
                        <p class="text-red-600 text-sm">{msg}</p>
                    })}
                    {move || saved.get().then(|| view! {
                        <p class="text-green-600 text-sm">"✓ Enregistré avec succès"</p>
                    })}
                    <button type="submit" class="btn btn-primary px-6" disabled=move || saving.get()>
                        {move || if saving.get() { "Enregistrement..." } else { "Enregistrer" }}
                    </button>
                </form>
            </div>
        </div>
    }
}
