use leptos::*;

use crate::components::{footer::Footer, navbar::Navbar};
use crate::models::MenuCategory;
use crate::server::functions::{create_quote, get_menu};
use crate::models::CreateQuotePayload;

#[component]
pub fn DevisPage() -> impl IntoView {
    let menu = create_resource(|| (), |_| get_menu());

    // Form fields
    let (last_name,        set_last_name)        = create_signal(String::new());
    let (first_name,       set_first_name)        = create_signal(String::new());
    let (phone,            set_phone)             = create_signal(String::new());
    let (email,            set_email)             = create_signal(String::new());
    let (event_date,       set_event_date)        = create_signal(String::new());
    let (event_place,      set_event_place)       = create_signal(String::new());
    let (number_of_people, set_number_of_people)  = create_signal(String::from("10"));
    let (main_dish,        set_main_dish)         = create_signal(String::new());
    let (starters,         set_starters)          = create_signal::<Vec<String>>(vec![]);
    let (desserts,         set_desserts)          = create_signal::<Vec<String>>(vec![]);
    let (message,          set_message)           = create_signal(String::new());

    let (submitting, set_submitting) = create_signal(false);
    let (success,    set_success)    = create_signal(false);
    let (error_msg,  set_error_msg)  = create_signal::<Option<String>>(None);

    let toggle_item = |list: ReadSignal<Vec<String>>, set_list: WriteSignal<Vec<String>>, id: String| {
        set_list.update(|v| {
            if v.contains(&id) {
                v.retain(|x| x != &id);
            } else {
                v.push(id);
            }
        });
        let _ = list;
    };

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if submitting.get() { return; }

        let n: u32 = number_of_people.get().parse().unwrap_or(0);
        if n == 0 {
            set_error_msg.set(Some("Veuillez indiquer un nombre de personnes valide.".into()));
            return;
        }
        if main_dish.get().is_empty() {
            set_error_msg.set(Some("Veuillez choisir un plat principal.".into()));
            return;
        }

        set_submitting.set(true);
        set_error_msg.set(None);

        let payload = CreateQuotePayload {
            last_name:        last_name.get(),
            first_name:       first_name.get(),
            phone:            phone.get(),
            email:            email.get(),
            event_date:       event_date.get(),
            event_place:      event_place.get(),
            number_of_people: n,
            starters:         starters.get(),
            main_dish:        main_dish.get(),
            desserts:         desserts.get(),
            message:          if message.get().is_empty() { None } else { Some(message.get()) },
        };

        spawn_local(async move {
            match create_quote(payload).await {
                Ok(_) => {
                    set_success.set(true);
                }
                Err(e) => {
                    set_error_msg.set(Some(format!("{e}")));
                }
            }
            set_submitting.set(false);
        });
    };

    view! {
        <div class="min-h-screen flex flex-col bg-creme-50">
            <Navbar />

            <main class="flex-1 pt-24 pb-16">
                <div class="max-w-3xl mx-auto px-6">

                    // Header
                    <div class="text-center mb-12">
                        <span class="section-label text-safran-600 block mb-3">"Gratuit & sans engagement"</span>
                        <h1 class="font-display text-4xl md:text-5xl text-ardoise-800 mb-4">"Votre devis 🍽️"</h1>
                        <p class="font-body text-ardoise-500 max-w-xl mx-auto">
                            "Décrivez votre événement et composez votre menu. "
                            "On vous répond rapidement avec une proposition personnalisée."
                        </p>
                    </div>

                    <Show when=move || success.get() fallback=|| ()>
                        <div class="bg-green-50 border border-green-200 rounded-3xl p-10 text-center animate-fade-in">
                            <div class="text-6xl mb-4">"🥘🎉"</div>
                            <h2 class="font-display text-3xl text-green-800 mb-2">"Votre demande est envoyée !"</h2>
                            <p class="font-body text-green-700">"On vous recontacte dans les plus brefs délais."</p>
                        </div>
                    </Show>

                    <Show when=move || !success.get() fallback=|| ()>
                        <form
                            on:submit=on_submit
                            class="bg-white rounded-3xl shadow-lg p-8 space-y-8"
                        >
                            // Informations personnelles
                            <fieldset class="space-y-5">
                                <legend class="font-hand text-2xl font-bold text-ardoise-800 mb-1">"Vos coordonnées"</legend>
                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                    <div>
                                        <label class="form-label">"Nom *"</label>
                                        <input
                                            type="text"
                                            class="form-input"
                                            placeholder="Dupont"
                                            required
                                            prop:value=last_name
                                            on:input=move |e| set_last_name.set(event_target_value(&e))
                                        />
                                    </div>
                                    <div>
                                        <label class="form-label">"Prénom *"</label>
                                        <input
                                            type="text"
                                            class="form-input"
                                            placeholder="Marie"
                                            required
                                            prop:value=first_name
                                            on:input=move |e| set_first_name.set(event_target_value(&e))
                                        />
                                    </div>
                                </div>
                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                    <div>
                                        <label class="form-label">"Téléphone"</label>
                                        <input
                                            type="tel"
                                            class="form-input"
                                            placeholder="06 00 00 00 00"
                                            prop:value=phone
                                            on:input=move |e| set_phone.set(event_target_value(&e))
                                        />
                                    </div>
                                    <div>
                                        <label class="form-label">"Email *"</label>
                                        <input
                                            type="email"
                                            class="form-input"
                                            placeholder="marie@example.fr"
                                            required
                                            prop:value=email
                                            on:input=move |e| set_email.set(event_target_value(&e))
                                        />
                                    </div>
                                </div>
                            </fieldset>

                            <hr class="border-creme-200" />

                            // Événement
                            <fieldset class="space-y-5">
                                <legend class="font-hand text-2xl font-bold text-ardoise-800 mb-1">"Votre événement"</legend>
                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                    <div>
                                        <label class="form-label">"Date de l'événement"</label>
                                        <input
                                            type="date"
                                            class="form-input"
                                            prop:value=event_date
                                            on:input=move |e| set_event_date.set(event_target_value(&e))
                                        />
                                    </div>
                                    <div>
                                        <label class="form-label">"Nombre de personnes *"</label>
                                        <input
                                            type="number"
                                            class="form-input"
                                            min="1"
                                            required
                                            prop:value=number_of_people
                                            on:input=move |e| set_number_of_people.set(event_target_value(&e))
                                        />
                                    </div>
                                </div>
                                <div>
                                    <label class="form-label">"Lieu de l'événement"</label>
                                    <input
                                        type="text"
                                        class="form-input"
                                        placeholder="Salle des fêtes, Paris 75001..."
                                        prop:value=event_place
                                        on:input=move |e| set_event_place.set(event_target_value(&e))
                                    />
                                </div>
                            </fieldset>

                            <hr class="border-creme-200" />

                            // Sélection du menu
                            <fieldset>
                                <legend class="font-hand text-2xl font-bold text-ardoise-800 mb-4">"Composition du menu 🥘"</legend>
                                <Suspense fallback=move || view! {
                                    <div class="flex justify-center py-8">
                                        <div class="w-8 h-8 border-4 border-bordeaux-700 border-t-transparent rounded-full animate-spin"></div>
                                    </div>
                                }>
                                    {move || menu.get().map(|result: Result<crate::models::MenuData, _>| match result {
                                        Err(_) => view! {
                                            <p class="text-cream-500 text-sm italic py-4">
                                                "La carte n'est pas encore disponible. Précisez vos souhaits dans le message."
                                            </p>
                                        }.into_view(),
                                        Ok(data) if data.items.is_empty() => view! {
                                            <p class="text-cream-500 text-sm italic py-4">
                                                "La carte n'est pas encore disponible. Précisez vos souhaits dans le message."
                                            </p>
                                        }.into_view(),
                                        Ok(data) => {
                                            let starter_items: Vec<_> = data.items.iter().filter(|i| i.category == MenuCategory::Starter).cloned().collect();
                                            let main_items: Vec<_>    = data.items.iter().filter(|i| i.category == MenuCategory::MainDish).cloned().collect();
                                            let dessert_items: Vec<_> = data.items.iter().filter(|i| i.category == MenuCategory::Dessert).cloned().collect();

                                            view! {
                                                <div class="space-y-6">
                                                    // Entrées (multi)
                                                    {if !starter_items.is_empty() { view! {
                                                        <div>
                                                            <p class="form-label">"🥗 Entrées (choix multiples)"</p>
                                                            <div class="flex flex-wrap gap-2">
                                                                {starter_items.into_iter().map(|item| {
                                                                    let id = item.id.clone();
                                                                    let id2 = id.clone();
                                                                    view! {
                                                                        <button
                                                                            type="button"
                                                                            class=move || {
                                                                                if starters.get().contains(&id) {
                                                                                    "tag bg-primary-600 text-white cursor-pointer"
                                                                                } else {
                                                                                    "tag bg-cream-100 text-cream-700 cursor-pointer hover:bg-primary-100"
                                                                                }
                                                                            }
                                                                            on:click=move |_| toggle_item(starters, set_starters, id2.clone())
                                                                        >
                                                                            {item.name}
                                                                        </button>
                                                                    }
                                                                }).collect_view()}
                                                            </div>
                                                        </div>
                                                    }.into_view()} else { view! {<div></div>}.into_view() }}

                                                    // Plat principal (unique)
                                                    {if !main_items.is_empty() { view! {
                                                        <div>
                                                            <p class="form-label">"🍲 Plat principal *"</p>
                                                            <div class="flex flex-wrap gap-2">
                                                                {main_items.into_iter().map(|item| {
                                                                    let id = item.id.clone();
                                                                    let id2 = id.clone();
                                                                    view! {
                                                                        <button
                                                                            type="button"
                                                                            class=move || {
                                                                                if main_dish.get() == id {
                                                                                    "tag bg-wine-700 text-white cursor-pointer"
                                                                                } else {
                                                                                    "tag bg-cream-100 text-cream-700 cursor-pointer hover:bg-wine-100"
                                                                                }
                                                                            }
                                                                            on:click=move |_| set_main_dish.set(id2.clone())
                                                                        >
                                                                            {item.name}
                                                                        </button>
                                                                    }
                                                                }).collect_view()}
                                                            </div>
                                                        </div>
                                                    }.into_view()} else { view! {<div></div>}.into_view() }}

                                                    // Desserts (multi)
                                                    {if !dessert_items.is_empty() { view! {
                                                        <div>
                                                            <p class="form-label">"🍮 Desserts (choix multiples)"</p>
                                                            <div class="flex flex-wrap gap-2">
                                                                {dessert_items.into_iter().map(|item| {
                                                                    let id = item.id.clone();
                                                                    let id2 = id.clone();
                                                                    view! {
                                                                        <button
                                                                            type="button"
                                                                            class=move || {
                                                                                if desserts.get().contains(&id) {
                                                                                    "tag bg-gold-500 text-cream-900 cursor-pointer"
                                                                                } else {
                                                                                    "tag bg-cream-100 text-cream-700 cursor-pointer hover:bg-gold-100"
                                                                                }
                                                                            }
                                                                            on:click=move |_| toggle_item(desserts, set_desserts, id2.clone())
                                                                        >
                                                                            {item.name}
                                                                        </button>
                                                                    }
                                                                }).collect_view()}
                                                            </div>
                                                        </div>
                                                    }.into_view()} else { view! {<div></div>}.into_view() }}
                                                </div>
                                            }.into_view()
                                        }
                                    })}
                                </Suspense>
                            </fieldset>

                            <hr class="border-cream-200" />

                            // Message libre
                            <div>
                                <label class="form-label">"Message complémentaire"</label>
                                <textarea
                                    class="form-input min-h-[100px] resize-y"
                                    placeholder="Allergies, demandes particulières, style de service..."
                                    prop:value=message
                                    on:input=move |e| set_message.set(event_target_value(&e))
                                ></textarea>
                            </div>

                            // Erreur
                            {move || error_msg.get().map(|msg| view! {
                                <div class="bg-red-50 border border-red-200 text-red-700 rounded-xl px-4 py-3 text-sm">
                                    {msg}
                                </div>
                            })}

                            // Bouton
                            <button
                                type="submit"
                                class="btn btn-safran w-full justify-center text-base py-4 shadow-lg"
                                disabled=move || submitting.get()
                            >
                                {move || if submitting.get() {
                                    "Envoi en cours...".to_string()
                                } else {
                                    "🍽️ Envoyer ma demande de devis".to_string()
                                }}
                            </button>
                        </form>
                    </Show>
                </div>
            </main>

            <Footer />
        </div>
    }
}
