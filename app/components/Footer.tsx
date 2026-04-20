import Link from "next/link";
import Image from "next/image";

export function Footer() {
  return (
    <footer className="bg-ardoise-800 pb-6 pt-16 text-ardoise-300">
      <div className="mx-auto max-w-6xl px-6">
        <div className="mb-12 grid grid-cols-1 gap-12 md:grid-cols-3">
          <div className="flex flex-col items-start gap-4">
            <div className="flex items-center gap-4">
              <Image src="/Logo.png" alt="La Poêlée du Bonheur" width={90} height={90}/>
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
              <span className="tag bg-bordeaux-100 text-bordeaux-700">🥘 Poêlées variées</span>
              <span className="tag bg-safran-100 text-safran-700">🥗 Entrées maison</span>
              <span className="tag bg-creme-200 text-ardoise-700">🍮 Desserts maison</span>
              <span className="tag bg-creme-200 text-ardoise-700">🌿 Produits frais</span>
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
                  Qui sommes-nous ?
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
                  href="mailto:lapoeleedubonheur@gmail.com"
                  className="flex items-center gap-2 font-body text-sm text-ardoise-400 transition-colors hover:text-safran-400"
                >
                  ✉️ lapoeleedubonheur@gmail.com
                </a>
              </li>
              <li>
                <a
                  href="https://www.facebook.com/people/La-po%C3%AAl%C3%A9e-du-bonheur/61587706023651/"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center gap-2 font-body text-sm text-ardoise-400 transition-colors hover:text-safran-400"
                >
                  📘 Facebook
                </a>
              </li>
            </ul>
          </div>
        </div>
        <div className="border-t border-white/10 pt-6 text-center font-body text-xs text-ardoise-600"> © {new Date().getFullYear()} La Poêlée du Bonheur.</div>
      </div>
    </footer>
  );
}
