import Link from "next/link";

export function Footer() {
  return (
    <footer className="bg-ardoise-800 pb-6 pt-16 text-ardoise-300">
      <div className="mx-auto max-w-6xl px-6">
        <div className="mb-12 grid grid-cols-1 gap-12 md:grid-cols-3">
          <div className="flex flex-col items-start gap-4">
            <div className="flex items-center gap-4">
              {/* eslint-disable-next-line @next/next/no-img-element */}
              <img
                src="/Logo.png"
                alt="La Poêlée du Bonheur"
                className="h-[90px] w-auto"
                onError={(e) => {
                  e.currentTarget.src = "/logo.png";
                }}
              />
              <p className="font-display text-2xl leading-tight text-white">
                La Poêlée
                <br />
                <span className="text-safran-400">du Bonheur</span>
              </p>
            </div>
            <p className="text-sm leading-relaxed text-ardoise-400">
              Traiteur événementielle spécialisée en paella et cuisine méditerranéenne. Fait maison,
              avec amour.
            </p>
            <div className="flex flex-wrap gap-2">
              <span className="tag bg-bordeaux-700/30 text-bordeaux-300">🥘 Paella</span>
              <span className="tag bg-safran-500/20 text-safran-400">🦐 Fruits de mer</span>
            </div>
          </div>
          <div>
            <h4 className="mb-5 font-body text-xs font-bold uppercase tracking-widest text-safran-400">
              Navigation
            </h4>
            <ul className="m-0 flex list-none flex-col gap-3 p-0">
              <li>
                <Link
                  href="/#about"
                  className="font-body text-sm text-ardoise-400 transition-colors hover:text-safran-400"
                >
                  Notre histoire
                </Link>
              </li>
              <li>
                <Link
                  href="/#menu"
                  className="font-body text-sm text-ardoise-400 transition-colors hover:text-safran-400"
                >
                  Nos plats
                </Link>
              </li>
              <li>
                <Link href="/devis" className="font-body text-sm text-ardoise-400 transition-colors hover:text-safran-400">
                  Demander un devis
                </Link>
              </li>
            </ul>
          </div>
          <div>
            <h4 className="mb-5 font-body text-xs font-bold uppercase tracking-widest text-safran-400">
              Contact
            </h4>
            <ul className="m-0 flex list-none flex-col gap-3 p-0">
              <li>
                <a
                  href="tel:0745852654"
                  className="flex items-center gap-2 font-body text-sm text-ardoise-400 transition-colors hover:text-safran-400"
                >
                  📞 07.45.85.26.54
                </a>
              </li>
              <li>
                <a
                  href="mailto:contact@lapoeleedubonheur.fr"
                  className="flex items-center gap-2 font-body text-sm text-ardoise-400 transition-colors hover:text-safran-400"
                >
                  ✉️ contact@lapoeleedubonheur.fr
                </a>
              </li>
            </ul>
          </div>
        </div>
        <div className="border-t border-white/10 pt-6 text-center font-body text-xs text-ardoise-600">
          © {new Date().getFullYear()} La Poêlée du Bonheur — Site réalisé avec ❤️ (Next.js).
        </div>
      </div>
    </footer>
  );
}
