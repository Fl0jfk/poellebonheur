"use client";

import Link from "next/link";
import Image from "next/image";
import { useEffect, useMemo, useState } from "react";
import { Footer } from "@/app/components/Footer";
import { Navbar } from "@/app/components/Navbar";
import { PhotoCollage } from "@/app/components/PhotoCollage";
type MenuCategoryNorm = "starter" | "main_dish" | "dessert";
type MarketEntry = { id: string; date: string; place: string };
type MarketsData = { markets: MarketEntry[] };
type MenuItem = {
  id: string;
  name: string;
  description: string;
  photo_url?: string | null;
  category: string;
};
type MenuData = { items: MenuItem[] };

const CATS: MenuCategoryNorm[] = ["starter", "main_dish", "dessert"];

function menuJsonUrl() {
  return "/api/public/menu";
}

function marketJsonUrl() {
  return "/api/public/market";
}

function normalizeMenuCategory(raw: string): MenuCategoryNorm {
  const lower = raw.trim().toLowerCase().replace(/-/g, "_");
  if (lower === "maindish" || lower === "main_dish") return "main_dish";
  if (lower === "dessert" || lower === "desserts") return "dessert";
  return "starter";
}

function categoryLabel(c: MenuCategoryNorm) {
  if (c === "main_dish") return "Plat principal";
  if (c === "dessert") return "Dessert";
  return "Entrée";
}

function categoryEmoji(c: MenuCategoryNorm) {
  if (c === "main_dish") return "🍲";
  if (c === "dessert") return "🍮";
  return "🥗";
}

function formatMarketDate(dateText: string): string {
  const dt = new Date(dateText);
  if (Number.isNaN(dt.getTime())) return dateText;
  return dt.toLocaleDateString("fr-FR", { weekday: "long", day: "numeric", month: "long" });
}

function filterUpcomingMarketsWithinWeek(entries: MarketEntry[]): MarketEntry[] {
  const start = new Date();
  start.setHours(0, 0, 0, 0);
  const end = new Date(start);
  end.setDate(end.getDate() + 7);
  end.setHours(23, 59, 59, 999);
  return entries
    .filter((m) => {
      const d = new Date(m.date);
      if (Number.isNaN(d.getTime())) return false;
      return d >= start && d <= end;
    })
    .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime());
}

export default function HomePage() {
  const [marketsData, setMarketsData] = useState<MarketsData | null | undefined>(undefined);
  const [menu, setMenu] = useState<MenuData | null | undefined>(undefined);
  const visibleMarkets = useMemo(() => {
    if (!marketsData?.markets) return [];
    return filterUpcomingMarketsWithinWeek(marketsData.markets);
  }, [marketsData]);
  const marketAnnouncement =
    visibleMarkets.length > 0 ? (
      <div className="mx-auto max-w-6xl px-3 py-2.5 text-center sm:px-4 sm:py-3">
        <div className="flex flex-col items-center gap-1.5 sm:flex-row sm:justify-center sm:gap-3">
          <span className="text-lg leading-none sm:text-xl">🛖</span>
          <p className="font-body text-sm font-medium leading-snug">
            Je serai présente au marché le <strong>{formatMarketDate(visibleMarkets[0].date)}</strong>
            {visibleMarkets[0].place ? ` à ${visibleMarkets[0].place}` : ""}.
          </p>
        </div>
        {visibleMarkets.length > 1 ? (
          <ul className="mt-2 space-y-1 border-t border-white/20 pt-2 text-xs font-medium text-white/90 sm:text-sm">
            {visibleMarkets.slice(1).map((m) => (
              <li key={m.id}>
                Également le {formatMarketDate(m.date)}
                {m.place ? ` — ${m.place}` : ""}
              </li>
            ))}
          </ul>
        ) : null}
      </div>
    ) : null;
  useEffect(() => {
    let ok = true;
    (async () => {
      const [mRes, menuRes] = await Promise.all([fetch(marketJsonUrl()), fetch(menuJsonUrl())]);
      if (!ok) return;
      if (mRes.ok) {
        const raw = (await mRes.json()) as MarketsData;
        setMarketsData(Array.isArray(raw.markets) ? raw : { markets: [] });
      } else {
        setMarketsData({ markets: [] });
      }
      setMenu(menuRes.ok ? ((await menuRes.json()) as MenuData) : { items: [] });
    })();
    return () => {
      ok = false;
    };
  }, []);
  return (
    <div className="flex min-h-screen flex-col">
      <Navbar announcement={marketAnnouncement} />
      <section
        className={
          visibleMarkets.length > 0
            ? visibleMarkets.length > 1
              ? "relative flex min-h-screen items-center justify-center pt-[calc(env(safe-area-inset-top,0px)+8.5rem)]"
              : "relative flex min-h-screen items-center justify-center pt-[calc(env(safe-area-inset-top,0px)+7.5rem)]"
            : "relative flex min-h-screen items-center justify-center pt-[calc(env(safe-area-inset-top,0px)+3.75rem)]"
        }
      >
        <div className="absolute inset-0 overflow-hidden">
          <div className="absolute inset-0 bg-gradient-to-br from-ardoise-700 via-ardoise-800 to-ardoise-900" />
          <div
            className="absolute inset-0 opacity-5"
            style={{
              backgroundImage: "radial-gradient(circle, #e8a030 1px, transparent 1px)",
              backgroundSize: "40px 40px",
            }}
          />
          <div className="absolute left-10 top-20 h-72 w-72 rounded-full bg-bordeaux-700/20 blur-3xl" />
          <div className="absolute bottom-20 right-10 h-96 w-96 rounded-full bg-safran-500/15 blur-3xl" />
        </div>
        <div className="relative z-10 mx-auto w-full max-w-4xl text-center">
          <div
            className={
              visibleMarkets.length > 0 ? "flex justify-center px-0 pt-2 max-sm:pt-4 sm:px-6" : "flex justify-center px-0 sm:px-6"
            }
          >
            <PhotoCollage hasMarketBanner={visibleMarkets.length > 0} />
          </div>
          <div className="pb-4 md:mt-12">
            <h1 className="mb-4 font-display text-5xl text-white drop-shadow-lg md:text-7xl">
              La Poêlée
              <br />
              <span className="text-safran-400">du Bonheur</span>
            </h1>
            <p className="mb-3 font-hand text-2xl text-ardoise-300">Traiteur événementielle</p>
            <p className="mx-auto mb-8 max-w-2xl font-body text-lg leading-relaxed text-ardoise-300">
              Paella géante, fruits de mer et saveurs méditerranéennes cuisinés avec passion pour vos
              mariages, anniversaires et marchés. 🦐🥘
            </p>
            <div className="flex flex-col items-center justify-center gap-4 sm:flex-row">
              <Link href="/devis" className="btn btn-safran px-10 py-4 text-base shadow-lg">
                🍽️ Demander un devis
              </Link>
              <Link href="#menu" className="btn btn-ghost px-8 py-4 text-base">
                Voir notre carte
              </Link>
            </div>
          </div>
        </div>
      </section>
      <section className="bg-bordeaux-700 py-16 text-white">
        <div className="mx-auto max-w-5xl px-6">
          <div className="grid grid-cols-2 gap-8 text-center md:grid-cols-4">
            {[
              ["100%", "Fait maison"],
              ["🦐", "Fruits de mer frais"],
              ["🥘", "Paella géante"],
              ["❤️", "Avec passion"],
            ].map(([a, b]) => (
              <div key={b}>
                <p className="mb-1 font-display text-4xl text-safran-400">{a}</p>
                <p className="font-body text-sm text-bordeaux-200">{b}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section id="about" className="bg-creme-50 py-24">
        <div className="mx-auto max-w-6xl px-6">
          <div className="grid grid-cols-1 items-center gap-16 md:grid-cols-2">
            <div className="relative flex justify-center">
              <div className="relative">
                <Image
                  src="/Logo.png"
                  alt="La Poêlée du Bonheur"
                  className="drop-shadow-xl"
                  width={300}
                  height={300}      
                />
                <div className="absolute -bottom-4 -right-4 rounded-2xl bg-safran-500 px-5 py-3 text-white shadow-lg">
                  <p className="font-hand text-lg font-bold">Devis gratuit !</p>
                </div>
                <div className="absolute -left-4 -top-4 rounded-2xl bg-bordeaux-700 px-4 py-2 text-white shadow-lg">
                  <p className="font-hand text-base">🦐 Fait maison</p>
                </div>
              </div>
            </div>
            <div>
              <span className="section-label mb-3 block text-safran-600">Qui sommes-nous ?</span>
              <h2 className="section-title mb-6 leading-tight text-ardoise-800">
                Une cuisine <span className="text-bordeaux-700">généreuse</span>
                <br />
                et conviviale
              </h2>
              <p className="mb-4 font-body leading-relaxed text-ardoise-600">
                La Poêlée du Bonheur, c&apos;est la promesse d&apos;une paella authentique et
                généreuse, préparée avec des produits frais de qualité. Nous nous déplaçons pour
                sublimer tous vos événements : mariages, anniversaires, fêtes de famille,
                séminaires...
              </p>
              <p className="mb-8 font-body leading-relaxed text-ardoise-600">
                Retrouvez-nous aussi sur les marchés locaux pour un avant-goût de bonheur !
              </p>
              <div className="flex flex-wrap gap-3">
                <span className="tag bg-bordeaux-100 text-bordeaux-700">🥘 Paella géante</span>
                <span className="tag bg-safran-100 text-safran-700">🦐 Fruits de mer</span>
                <span className="tag bg-creme-200 text-ardoise-700">🫒 Méditerranéen</span>
                <span className="tag bg-creme-200 text-ardoise-700">🌿 Produits frais</span>
              </div>
            </div>
          </div>
        </div>
      </section>
      <section id="menu" className="bg-white py-24">
        <div className="mx-auto max-w-6xl px-6">
          <div className="mb-14 text-center">
            <span className="section-label mb-3 block text-safran-600">Notre carte</span>
            <h2 className="section-title mb-4 text-ardoise-800">
              Des saveurs <span className="text-bordeaux-700">qui régalent</span>
            </h2>
            <p className="mx-auto max-w-xl font-body text-ardoise-500">
              Chaque plat est préparé le jour même avec des ingrédients frais et de saison.
            </p>
          </div>
          {menu === undefined ? (
            <div className="flex justify-center py-16">
              <div className="h-12 w-12 animate-spin rounded-full border-4 border-bordeaux-700 border-t-transparent" />
            </div>
          ) : menu === null ? (
            <div className="px-4 py-16 text-center">
              <p className="font-body text-ardoise-600">
                La carte ne peut pas être chargée pour le moment (réseau ou configuration).
              </p>
            </div>
          ) : menu.items.length === 0 ? (
            <div className="py-16 text-center">
              <div className="mb-4 text-6xl">🥘</div>
              <p className="font-hand text-2xl text-ardoise-500">La carte est en cours de préparation...</p>
            </div>
          ) : (
            <div className="space-y-16">
              {CATS.map((cat) => {
                const catItems = menu.items.filter(
                  (i) => normalizeMenuCategory(i.category) === cat,
                );
                if (catItems.length === 0) return null;
                return (
                  <div key={cat}>
                    <h3 className="mb-6 flex items-center gap-3 font-hand text-3xl font-bold text-ardoise-800">
                      <span className="text-4xl">{categoryEmoji(cat)}</span>
                      {categoryLabel(cat)}
                      <span className="ml-2 h-px flex-1 bg-creme-200" />
                    </h3>
                    <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                      {catItems.map((item) => (
                        <div
                          key={item.id}
                          className="card overflow-hidden border border-creme-100"
                        >
                          {item.photo_url ? (
                            <div className="relative h-48 w-full">
                              <Image src={item.photo_url} alt={item.name} fill className="object-cover"/>
                            </div>
                          ) : (
                            <div className="flex h-48 w-full items-center justify-center bg-creme-100 text-5xl text-ardoise-300">
                             🍽️
                            </div>
                          )}
                          <div className="p-5">
                            <h4 className="mb-1 font-hand text-xl font-bold text-ardoise-800">
                              {item.name}
                            </h4>
                            <p className="mb-3 font-body text-sm leading-relaxed text-ardoise-500">
                              {item.description}
                            </p>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                );
              })}
            </div>
          )}

          <div className="mt-14 text-center">
            <Link href="/devis" className="btn btn-primary px-10 py-4 text-base">
              🍽️ Composer mon menu sur mesure
            </Link>
          </div>
        </div>
      </section>

      <section className="relative overflow-hidden bg-gradient-to-br from-ardoise-800 to-ardoise-900 py-24 text-center text-white">
        <div
          className="absolute inset-0 opacity-10"
          style={{
            backgroundImage: "radial-gradient(circle, #e8a030 1px, transparent 1px)",
            backgroundSize: "50px 50px",
          }}
        />
        <div className="absolute left-1/2 top-0 h-96 w-96 -translate-x-1/2 rounded-full bg-bordeaux-700/20 blur-3xl" />
        <div className="relative mx-auto max-w-2xl px-6">
          <div className="mb-6 text-6xl">🥘</div>
          <h2 className="mb-4 font-display text-4xl md:text-5xl">
            Votre événement,
            <br />
            <span className="text-safran-400">notre bonheur !</span>
          </h2>
          <p className="mb-10 font-body text-lg leading-relaxed text-ardoise-300">
            Parlez-nous de votre projet. On vous prépare un devis gratuit et personnalisé.
          </p>
          <div className="flex flex-col justify-center gap-4 sm:flex-row">
            <Link href="/devis" className="btn btn-safran px-10 py-4 text-base shadow-xl">
              Demander un devis gratuit
            </Link>
            <a href="tel:0745852654" className="btn btn-white px-8 py-4 text-base">
              📞 07.45.85.26.54
            </a>
          </div>
        </div>
      </section>
      <Footer />
    </div>
  );
}
