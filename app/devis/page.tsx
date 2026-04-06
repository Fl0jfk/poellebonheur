"use client";

import { useEffect, useState } from "react";
import Image from "next/image";
import { Footer } from "@/app/components/Footer";
import { Navbar } from "@/app/components/Navbar";

type MenuItem = {
  id: string;
  name: string;
  description: string;
  photo_url?: string | null;
  category: string;
};
type MenuData = { items: MenuItem[] };

function menuJsonUrl() {return "/api/public/menu";}

function normalizeMenuCategory(raw: string): "starter" | "main_dish" | "dessert" {
  const lower = raw.trim().toLowerCase().replace(/-/g, "_");
  if (lower === "maindish" || lower === "main_dish") return "main_dish";
  if (lower === "dessert" || lower === "desserts") return "dessert";
  return "starter";
}

async function fetchMenu() {
  const r = await fetch(menuJsonUrl());
  if (!r.ok) return null;
  return (await r.json()) as MenuData;
}

async function sendDevis(body: Record<string, unknown>) {
  const r = await fetch("/api/send-devis", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  if (!r.ok) {
    const text = await r.text();
    throw new Error(text || `Erreur HTTP ${r.status}`);
  }
}

function PickTile({
  item,
  selected,
  selectedFrame,
  idleFrame,
  onPick,
}: {
  item: MenuItem;
  selected: boolean;
  selectedFrame: string;
  idleFrame: string;
  onPick: () => void;
}) {
  const frame = selected ? selectedFrame : idleFrame;
  return (
    <button
      type="button"
      onClick={onPick}
      className={`flex h-full w-full min-w-0 flex-col overflow-hidden rounded-xl bg-white text-left shadow-sm transition-[box-shadow,transform] hover:-translate-y-0.5 hover:shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-bordeaux-500 focus-visible:ring-offset-2 ${frame}`}
    >
      <div className="relative aspect-[5/3] w-full shrink-0 bg-gradient-to-br from-creme-100 to-creme-200">
        {item.photo_url ? (
          <Image
            src={item.photo_url}
            alt={item.name}
            fill
            className="object-cover"
            sizes="(max-width: 640px) 45vw, 200px"
          />
        ) : (
          <span className="flex h-full w-full items-center justify-center text-2xl text-ardoise-300" aria-hidden>
            🍽️
          </span>
        )}
      </div>
      <div className="flex min-h-0 flex-1 flex-col gap-1 p-2.5 sm:p-3">
        <span className="font-body text-xs font-semibold leading-snug text-ardoise-800 sm:text-sm">
          {item.name}
        </span>
        {item.description?.trim() ? (
          <span className="line-clamp-3 font-body text-[11px] leading-snug text-ardoise-500 sm:text-xs">
            {item.description}
          </span>
        ) : null}
      </div>
    </button>
  );
}

export default function DevisPage() {
  const [menu, setMenu] = useState<MenuData | null | undefined>(undefined);
  const [lastName, setLastName] = useState("");
  const [firstName, setFirstName] = useState("");
  const [phone, setPhone] = useState("");
  const [email, setEmail] = useState("");
  const [eventDate, setEventDate] = useState("");
  const [eventPlace, setEventPlace] = useState("");
  const [numberOfPeople, setNumberOfPeople] = useState("10");
  const [mainDish, setMainDish] = useState("");
  const [starters, setStarters] = useState<string[]>([]);
  const [desserts, setDesserts] = useState<string[]>([]);
  const [message, setMessage] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [success, setSuccess] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  useEffect(() => {
    let ok = true;
    (async () => {
      const m = await fetchMenu();
      if (ok) setMenu(m ?? { items: [] });
    })();
    return () => {
      ok = false;
    };
  }, []);

  function toggleId(list: string[], id: string, set: (v: string[]) => void) {
    if (list.includes(id)) set(list.filter((x) => x !== id));
    else set([...list, id]);
  }

  async function onSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (submitting) return;
    const n = parseInt(numberOfPeople, 10);
    if (!n || n < 1) {
      setErrorMsg("Veuillez indiquer un nombre de personnes valide.");
      return;
    }
    if (!mainDish) {
      setErrorMsg("Veuillez choisir un plat principal.");
      return;
    }
    setSubmitting(true);
    setErrorMsg(null);
    try {
      await sendDevis({
        last_name: lastName,
        first_name: firstName,
        phone,
        email,
        event_date: eventDate,
        event_place: eventPlace,
        number_of_people: n,
        starters,
        main_dish: mainDish,
        desserts,
        message: message.trim() ? message : undefined,
      });
      setSuccess(true);
    } catch (err) {
      setErrorMsg(err instanceof Error ? err.message : "Erreur d’envoi");
    }
    setSubmitting(false);
  }
  const data = menu;
  const starterItems = data?.items.filter((i) => normalizeMenuCategory(i.category) === "starter") ?? [];
  const mainItems = data?.items.filter((i) => normalizeMenuCategory(i.category) === "main_dish") ?? [];
  const dessertItems = data?.items.filter((i) => normalizeMenuCategory(i.category) === "dessert") ?? [];
  return (
    <div className="flex min-h-screen flex-col">
      <Navbar />
      <main className="flex-1 pb-16 pt-24">
        <div className="mx-auto max-w-3xl px-6">
          <div className="mb-12 text-center">
            <span className="section-label mb-3 block text-safran-600">Gratuit & sans engagement</span>
            <h1 className="mb-4 font-display text-4xl text-ardoise-800 md:text-5xl">Votre devis 🍽️</h1>
            <p className="mx-auto max-w-xl font-body text-ardoise-500">Décrivez votre événement et composez votre menu. On vous répond rapidement avec une proposition personnalisée.</p>
          </div>
          {success ? (
            <div className="animate-fade-in rounded-3xl border border-green-200 bg-green-50 p-10 text-center">
              <div className="mb-4 text-6xl">🥘🎉</div>
              <h2 className="mb-2 font-display text-3xl text-green-800">Votre demande est envoyée !</h2>
              <p className="font-body text-green-700">On vous recontacte dans les plus brefs délais.</p>
            </div>
          ) : (
            <form onSubmit={onSubmit} className="space-y-8 rounded-3xl bg-white p-8 shadow-lg">
              <fieldset className="space-y-5">
                <legend className="mb-1 font-hand text-2xl font-bold text-ardoise-800">
                  Vos coordonnées
                </legend>
                <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                  <div>
                    <label className="form-label">Nom *</label>
                    <input
                      className="form-input"
                      required
                      value={lastName}
                      onChange={(e) => setLastName(e.target.value)}
                      placeholder="Dupont"
                    />
                  </div>
                  <div>
                    <label className="form-label">Prénom *</label>
                    <input
                      className="form-input"
                      required
                      value={firstName}
                      onChange={(e) => setFirstName(e.target.value)}
                      placeholder="Marie"
                    />
                  </div>
                </div>
                <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                  <div>
                    <label className="form-label">Téléphone *</label>
                    <input
                      className="form-input"
                      type="tel"
                      value={phone}
                      required

                      onChange={(e) => setPhone(e.target.value)}
                      placeholder="06 00 00 00 00"
                    />
                  </div>
                  <div>
                    <label className="form-label">Email *</label>
                    <input
                      className="form-input"
                      type="email"
                      required
                      value={email}
                      onChange={(e) => setEmail(e.target.value)}
                      placeholder="marie@example.fr"
                    />
                  </div>
                </div>
              </fieldset>
              <hr className="border-creme-200" />
              <fieldset className="space-y-5">
                <legend className="mb-1 font-hand text-2xl font-bold text-ardoise-800">Votre événement</legend>
                <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                  <div>
                    <label className="form-label">Date de l&apos;événement</label>
                    <input
                      className="form-input"
                      type="date"
                      value={eventDate}
                      onChange={(e) => setEventDate(e.target.value)}
                    />
                  </div>
                  <div>
                    <label className="form-label">Nombre de personnes *</label>
                    <input
                      className="form-input"
                      type="number"
                      min={1}
                      required
                      value={numberOfPeople}
                      onChange={(e) => setNumberOfPeople(e.target.value)}
                    />
                  </div>
                </div>
                <div>
                  <label className="form-label">Lieu de l&apos;événement</label>
                  <input
                    className="form-input"
                    value={eventPlace}
                    onChange={(e) => setEventPlace(e.target.value)}
                    placeholder="Salle des fêtes, Paris 75001..."
                  />
                </div>
              </fieldset>
              <hr className="border-creme-200" />
              <fieldset>
                <legend className="mb-4 font-hand text-2xl font-bold text-ardoise-800">
                  Composition du menu 🥘
                </legend>
                {menu === undefined ? (
                  <div className="flex justify-center py-8">
                    <div className="h-8 w-8 animate-spin rounded-full border-4 border-bordeaux-700 border-t-transparent" />
                  </div>
                ) : menu === null ? (
                  <p className="py-4 text-sm text-red-600">
                    Impossible de charger la carte (réseau ou configuration). Vous pouvez décrire
                    votre menu dans le message.
                  </p>
                ) : menu.items.length === 0 ? (
                  <p className="py-4 text-sm italic text-ardoise-500">
                    La carte n&apos;est pas encore disponible. Précisez vos souhaits dans le message.
                  </p>
                ) : (
                  <div className="space-y-6">
                    {starterItems.length > 0 ? (
                      <div>
                        <p className="form-label">🥗 Entrées (choix multiples)</p>
                        <div className="grid grid-cols-2 gap-3 sm:grid-cols-3">
                          {starterItems.map((item) => (
                            <PickTile
                              key={item.id}
                              item={item}
                              selected={starters.includes(item.id)}
                              selectedFrame="ring-2 ring-bordeaux-400 border border-bordeaux-600"
                              idleFrame="border border-creme-200 hover:border-bordeaux-200"
                              onPick={() => toggleId(starters, item.id, setStarters)}
                            />
                          ))}
                        </div>
                      </div>
                    ) : null}
                    {mainItems.length > 0 ? (
                      <div>
                        <p className="form-label">🍲 Plat principal *</p>
                        <div className="grid grid-cols-2 gap-3 sm:grid-cols-3">
                          {mainItems.map((item) => (
                            <PickTile
                              key={item.id}
                              item={item}
                              selected={mainDish === item.id}
                              selectedFrame="ring-2 ring-bordeaux-500 border border-bordeaux-700"
                              idleFrame="border border-creme-200 hover:border-bordeaux-200"
                              onPick={() => setMainDish(item.id)}
                            />
                          ))}
                        </div>
                      </div>
                    ) : null}
                    {dessertItems.length > 0 ? (
                      <div>
                        <p className="form-label">🍮 Desserts (choix multiples)</p>
                        <div className="grid grid-cols-2 gap-3 sm:grid-cols-3">
                          {dessertItems.map((item) => (
                            <PickTile
                              key={item.id}
                              item={item}
                              selected={desserts.includes(item.id)}
                              selectedFrame="ring-2 ring-safran-400 border border-safran-600"
                              idleFrame="border border-creme-200 hover:border-safran-200"
                              onPick={() => toggleId(desserts, item.id, setDesserts)}
                            />
                          ))}
                        </div>
                      </div>
                    ) : null}
                  </div>
                )}
              </fieldset>
              <hr className="border-creme-200" />
              <div>
                <label className="form-label">Message complémentaire</label>
                <textarea
                  className="form-input min-h-[100px] resize-y"
                  value={message}
                  onChange={(e) => setMessage(e.target.value)}
                  placeholder="Allergies, demandes particulières, style de service..."
                />
              </div>
              {errorMsg ? (
                <div className="rounded-xl border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
                  {errorMsg}
                </div>
              ) : null}
              <button
                type="submit"
                disabled={submitting}
                className="btn btn-safran w-full justify-center py-4 text-base shadow-lg"
              >
                {submitting ? "Envoi en cours..." : "🍽️ Envoyer ma demande de devis"}
              </button>
            </form>
          )}
        </div>
      </main>
      <section className="border-t border-creme-200 bg-gradient-to-b from-creme-50 to-white py-10">
        <div className="mx-auto max-w-3xl px-6 text-center">
          <p className="font-hand text-xl font-bold text-ardoise-700">Une question avant de réserver ?</p>
          <p className="mt-1 text-sm text-ardoise-500">Appelez-nous directement, on vous conseille avec plaisir.</p>
          <a href="tel:0745852654" className="mt-4 inline-flex items-center justify-center gap-2 rounded-full border-2 border-bordeaux-600 bg-bordeaux-700 px-8 py-3.5 font-body text-base font-semibold text-white shadow-md transition-colors hover:bg-bordeaux-800">
            📞 07.45.85.26.54
          </a>
        </div>
      </section>
      <Footer />
    </div>
  );
}
