"use client";

import Link from "next/link";
import Image from "next/image";
import { useState, type ReactNode } from "react";

type NavbarProps = {
  /** Bandeau au-dessus de la barre (ex. annonces marché), inclus dans le bloc fixe pour ne pas passer sous le header */
  announcement?: ReactNode;
};

export function Navbar({ announcement }: NavbarProps) {
  const [menuOpen, setMenuOpen] = useState(false);
  const hasAnnouncement = Boolean(announcement);
  return (
    <nav className="fixed left-0 right-0 top-0 z-50 flex flex-col">
      {announcement ? (
        <div className="relative z-[60] w-full shrink-0 border-b border-white/15 bg-bordeaux-700 text-white shadow-sm">
          {announcement}
        </div>
      ) : null}
      <div className="relative z-20 w-full bg-white/75 backdrop-blur-xl shadow-[0_1px_12px_rgba(0,0,0,0.08)]">
        <div className="mx-auto flex max-w-6xl items-center px-4 py-1.5 md:py-[8px]">
          <div className="grid w-full grid-cols-3 items-center md:flex md:justify-between md:gap-8">
            <div className="flex items-center">
              <Link
                href="/"
                className="md:hidden"
                onClick={() => setMenuOpen(false)}
              >
                {/* eslint-disable-next-line @next/next/no-img-element */}
                <Image
                  src="/Logo.png"
                  alt="La Poêlée du Bonheur"
                  width={64}
                  height={64}
                  className="h-[64px] w-[64px] object-contain"
                />
              </Link>
              <Link
                href="/"
                className="group hidden items-center gap-3 no-underline md:flex"
                onClick={() => setMenuOpen(false)}
              >
                {/* eslint-disable-next-line @next/next/no-img-element */}
                <Image
                  src="/Logo.png"
                  alt="La Poêlée du Bonheur"
                  className="block h-[60px] w-[60px] object-contain transition-transform group-hover:scale-105"
                  width={50}
                  height={50}
                  
                />
                <span className="font-display text-[1.8rem] leading-tight text-bordeaux-700">
                  La Poêlée du Bonheur
                </span>
              </Link>
            </div>
            <div className="flex justify-center md:hidden">
              <span className="whitespace-nowrap font-display text-[1.2rem] font-bold leading-tight text-bordeaux-700">
                La Poêlée du Bonheur
              </span>
            </div>
            <div className="flex items-center justify-end gap-6">
              <ul className="m-0 hidden list-none items-center gap-8 p-0 md:flex">
                <li>
                  <Link
                    href="/#about"
                    className="font-body text-sm font-semibold text-ardoise-700 transition-colors hover:text-bordeaux-700"
                  >
                    Notre histoire
                  </Link>
                </li>
                <li>
                  <Link
                    href="/#menu"
                    className="font-body text-sm font-semibold text-ardoise-700 transition-colors hover:text-bordeaux-700"
                  >
                    Nos plats
                  </Link>
                </li>
                <li>
                  <Link href="/devis" className="btn btn-safran px-5 py-2 text-sm">
                    🍽️ Réserver
                  </Link>
                </li>
              </ul>
              <button
                type="button"
                className="flex h-10 w-10 cursor-pointer select-none flex-col items-center justify-center gap-[5px] rounded-xl hover:bg-ardoise-100/60 md:hidden"
                aria-label="Menu"
                aria-expanded={menuOpen}
                onClick={() => setMenuOpen((o) => !o)}
              >
                <span
                  className={`block h-[2px] w-[22px] origin-center rounded-full bg-ardoise-700 transition-all duration-300 ${
                    menuOpen ? "translate-y-[7px] rotate-45" : ""
                  }`}
                />
                <span
                  className={`block h-[2px] w-[22px] rounded-full bg-ardoise-700 transition-all duration-300 ${
                    menuOpen ? "scale-x-0 opacity-0" : ""
                  }`}
                />
                <span
                  className={`block h-[2px] w-[22px] origin-center rounded-full bg-ardoise-700 transition-all duration-300 ${
                    menuOpen ? "-translate-y-[7px] -rotate-45" : ""
                  }`}
                />
              </button>
            </div>
          </div>
        </div>
      </div>

      <div
        id="nav-mobile"
        className={`fixed inset-0 z-10 box-border flex max-h-[100dvh] min-h-full flex-col overflow-y-auto bg-white transition-all duration-200 ease-out md:hidden ${
          hasAnnouncement
            ? "pt-[calc(env(safe-area-inset-top,0px)+63px+min(28vw,7.5rem))]"
            : "pt-[calc(env(safe-area-inset-top,0px)+63px)]"
        } ${
          menuOpen ? "pointer-events-auto translate-y-0 opacity-100" : "pointer-events-none -translate-y-2 opacity-0"
        }`}
      >
        <div className="flex flex-1 flex-col justify-center gap-2 px-6 py-8">
          <Link
            href="/#about"
            onClick={() => setMenuOpen(false)}
            className="flex items-center gap-4 rounded-2xl px-5 py-4 font-body text-lg font-semibold text-ardoise-800 no-underline transition-colors hover:bg-creme-100"
          >
            <span className="text-2xl">🏡</span>
            Notre histoire
          </Link>
          <Link
            href="/#menu"
            onClick={() => setMenuOpen(false)}
            className="flex items-center gap-4 rounded-2xl px-5 py-4 font-body text-lg font-semibold text-ardoise-800 no-underline transition-colors hover:bg-creme-100"
          >
            <span className="text-2xl">🥘</span>
            Nos plats
          </Link>
          <Link
            href="/devis"
            onClick={() => setMenuOpen(false)}
            className="flex items-center gap-4 rounded-2xl px-5 py-4 font-body text-lg font-semibold text-ardoise-800 no-underline transition-colors hover:bg-creme-100"
          >
            <span className="text-2xl">🍽️</span>
            Demander un devis
          </Link>
          <div className="my-2 border-t border-ardoise-100/60" />
          <a
            href="https://www.facebook.com/people/La-Po%C3%AAl%C3%A9e-du-Bonheur/61572905885666/"
            target="_blank"
            rel="noopener noreferrer"
            onClick={() => setMenuOpen(false)}
            className="flex items-center gap-4 rounded-2xl px-5 py-4 font-body text-lg font-semibold text-ardoise-800 no-underline transition-colors hover:bg-blue-50"
          >
            <span className="text-2xl">📘</span>
            Facebook
          </a>
          <a
            href="tel:0745852654"
            onClick={() => setMenuOpen(false)}
            className="flex items-center gap-4 rounded-2xl bg-bordeaux-50 px-5 py-4 font-body text-lg font-semibold text-bordeaux-700 no-underline transition-colors hover:bg-bordeaux-100"
          >
            <span className="text-2xl">📞</span>
            07.45.85.26.54
          </a>
        </div>
        <div className="shrink-0 border-t border-ardoise-100/40 px-6 py-6">
          <Link
            href="/devis"
            onClick={() => setMenuOpen(false)}
            className="btn btn-safran flex w-full justify-center py-4 text-base"
          >
            🍽️ Réserver un événement
          </Link>
        </div>
      </div>
    </nav>
  );
}
