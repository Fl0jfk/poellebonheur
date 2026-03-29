# Nusha Traiteur — Site vitrine

Site web événementiel pour traiteur, avec backend léger en Rust et stockage JSON sur AWS S3.

## Architecture

```
traiteur-website/
├── src/
│   ├── main.rs              # Point d'entrée, routeur Axum
│   ├── config.rs            # Configuration (variables d'environnement)
│   ├── models.rs            # Structures de données
│   ├── error.rs             # Gestion d'erreurs HTTP
│   ├── storage.rs           # Opérations S3 (lecture/écriture JSON, photos)
│   ├── auth_middleware.rs   # Middleware JWT
│   └── handlers/
│       ├── auth.rs          # Connexion admin
│       ├── market.rs        # Gestion du prochain marché
│       ├── menu.rs          # CRUD plats (entrées/plats/desserts)
│       └── quote.rs         # Demandes de devis
├── static/
│   ├── index.html           # Page d'accueil
│   ├── devis.html           # Formulaire de devis interactif
│   ├── admin/
│   │   ├── index.html       # Login admin
│   │   └── dashboard.html   # Dashboard admin
│   ├── css/style.css        # Styles
│   └── js/
│       ├── main.js          # Page d'accueil
│       ├── devis.js         # Formulaire de devis
│       └── admin.js         # Dashboard admin
└── Cargo.toml
```

## Données stockées sur S3

| Clé S3                  | Contenu                          |
|-------------------------|----------------------------------|
| `data/market.json`      | Infos prochain marché            |
| `data/menu.json`        | Tous les plats (entrées/plats/desserts) |
| `data/quotes.json`      | Demandes de devis reçues         |
| `photos/{uuid}.{ext}`   | Photos des plats                 |

## Prérequis

- [Rust](https://rustup.rs/) 1.75+
- Compte AWS avec un bucket S3

## Configuration AWS S3

### 1. Créer un bucket S3

```bash
aws s3 mb s3://nusha-traiteur-data --region eu-west-3
```

### 2. Configurer la politique du bucket pour les photos (accès public en lecture)

Dans la console AWS S3, onglet **Permissions** du bucket :

1. Désactiver "Block all public access" (pour le prefix `photos/`)
2. Ajouter cette politique :

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": "*",
      "Action": "s3:GetObject",
      "Resource": "arn:aws:s3:::nusha-traiteur-data/photos/*"
    }
  ]
}
```

### 3. Créer un utilisateur IAM

Créez un utilisateur IAM avec cette politique :

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "s3:GetObject",
        "s3:PutObject",
        "s3:DeleteObject",
        "s3:ListBucket"
      ],
      "Resource": [
        "arn:aws:s3:::nusha-traiteur-data",
        "arn:aws:s3:::nusha-traiteur-data/*"
      ]
    }
  ]
}
```

## Installation

### 1. Cloner et configurer

```bash
cp .env.example .env
# Éditez .env avec vos vraies valeurs
```

### 2. Générer le hash du mot de passe admin

```bash
python3 -c "import bcrypt; print(bcrypt.hashpw(b'votre-mot-de-passe', bcrypt.gensalt()).decode())"
```

Copiez le hash généré dans `ADMIN_PASSWORD_HASH` dans votre `.env`.

Ou avec `htpasswd` :
```bash
htpasswd -bnBC 12 "" "votre-mot-de-passe" | tr -d ':\n'
```

### 3. Compiler et lancer

```bash
cargo build --release
cargo run --release
```

Le serveur démarre sur `http://localhost:3000`.

## Routes API

### Publiques

| Méthode | Route          | Description                       |
|---------|---------------|-----------------------------------|
| GET     | `/api/market` | Infos prochain marché             |
| GET     | `/api/menu`   | Tous les plats                    |
| POST    | `/api/quotes` | Soumettre une demande de devis    |
| POST    | `/api/auth/login` | Connexion admin              |

### Admin (JWT requis)

| Méthode | Route                       | Description                     |
|---------|----------------------------|---------------------------------|
| PUT     | `/api/admin/market`        | Mettre à jour le prochain marché |
| POST    | `/api/admin/menu`          | Ajouter un plat                 |
| PUT     | `/api/admin/menu/:id`      | Modifier un plat                |
| DELETE  | `/api/admin/menu/:id`      | Supprimer un plat               |
| POST    | `/api/admin/menu/:id/photo`| Uploader une photo              |
| GET     | `/api/admin/quotes`        | Voir toutes les demandes        |

## Pages

| URL                    | Description                          |
|------------------------|--------------------------------------|
| `/`                    | Page d'accueil                       |
| `/devis.html`          | Formulaire de devis interactif       |
| `/admin/`              | Connexion admin                      |
| `/admin/dashboard.html`| Dashboard de gestion                 |

## Déploiement sur un serveur

```bash
# Build release
cargo build --release

# Copier le binaire et les fichiers statiques
cp target/release/traiteur-website /var/www/traiteur/
cp -r static/ /var/www/traiteur/

# Créer un fichier .env dans le répertoire de travail
# Puis lancer :
cd /var/www/traiteur && ./traiteur-website
```

Pour la production, utilisez un process manager comme `systemd` ou `supervisor`,
et un reverse proxy (nginx) devant le serveur Rust.

## Personnalisation

- **Nom de la traiteure** : Cherchez `Nusha Traiteur` dans les fichiers HTML pour personnaliser
- **Numéro de téléphone** : `static/index.html`, section footer et CTA
- **Email** : `static/index.html`, section footer
- **Couleurs** : Variables CSS dans `static/css/style.css` (section `:root`)
