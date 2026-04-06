# La Poêlée du Bonheur — site (Next.js)

Site vitrine + formulaire de devis + espace admin, en **Next.js** avec API routes natives.

## Prérequis

- Node.js 20+
- Fichiers médias dans `public/` : `Logo.png`, `Plat1.avif`, `Entree1.avif`, `Dessert1.avif`, `Repas2.avif`, `Repas3.avif` (remettez-les depuis une sauvegarde Git si besoin).

## Développement

```bash
npm install
cp .env.example .env.local
# Éditer .env.local (API Gateway, clés admin)
npm run dev
```

Ouvre [http://localhost:3000](http://localhost:3000).

## Build production

```bash
npm run build
```

Build Next standard (pas de dossier `out`).

## Variables d’environnement

Préfixe **`NEXT_PUBLIC_`** : injectées au **build** et visibles côté navigateur.

| Variable | Rôle |
|----------|------|
| `NEXT_PUBLIC_API_GATEWAY_URL` | Base API (ex. `https://xxx.execute-api.eu-west-3.amazonaws.com`) |
| `NEXT_PUBLIC_ADMIN_PASSWORD` | Mot de passe page `/admin` |
| `NEXT_PUBLIC_ADMIN_API_KEY` | Header `x-admin-key` pour les Lambdas admin |
| `NEXT_PUBLIC_S3_DATA_BASE_URL` | Optionnel : lecture directe des JSON sur S3 si pas d’API |

## API intégrée (Next.js)

Routes disponibles :
- `GET /api/public/menu`
- `GET /api/public/market`
- `POST /api/send-devis`
- `GET /api/admin/quotes`
- `POST /api/admin/menu` (actions `list`, `create`, `delete`)
- `POST /api/admin/market`
- `POST /api/admin/upload-photo`

Les données sont stockées localement dans `data/*.json`.

## Structure

- `app/` — pages (`/`, `/devis`, `/admin`) avec types + appels API directement dedans
- `components/` — barre de navigation, pied de page, collage
- `public/` — assets statiques, JSON locaux par défaut
