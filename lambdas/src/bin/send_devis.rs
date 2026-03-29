use anyhow::Result;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lettre::{
    message::header::ContentType, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use serde::Deserialize;
use traiteur_lambdas::{err, ok, preflight, s3_get, s3_put, QuoteRequest, QuotesData};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct DevisPayload {
    last_name:        String,
    first_name:       String,
    #[serde(default)]
    phone:            String,
    email:            String,
    #[serde(default)]
    event_date:       String,
    #[serde(default)]
    event_place:      String,
    number_of_people: u32,
    #[serde(default)]
    starters:         Vec<String>,
    main_dish:        String,
    #[serde(default)]
    desserts:         Vec<String>,
    message:          Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();
    run(service_fn(handler)).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if req.method() == lambda_http::http::Method::OPTIONS {
        return Ok(preflight());
    }

    let body_str = match req.body() {
        Body::Text(s)   => s.clone(),
        Body::Binary(b) => String::from_utf8(b.to_vec()).unwrap_or_default(),
        Body::Empty     => return Ok(err(400, "Corps de requête vide")),
    };

    let payload: DevisPayload = match serde_json::from_str(&body_str) {
        Ok(p) => p,
        Err(e) => return Ok(err(400, e)),
    };

    // Validation
    if payload.last_name.trim().is_empty() || payload.first_name.trim().is_empty() {
        return Ok(err(400, "Nom et prénom obligatoires"));
    }
    if payload.email.trim().is_empty() {
        return Ok(err(400, "Email obligatoire"));
    }
    if payload.main_dish.trim().is_empty() {
        return Ok(err(400, "Plat principal obligatoire"));
    }
    if payload.number_of_people == 0 {
        return Ok(err(400, "Nombre de personnes invalide"));
    }

    // ── 1. Sauvegarder dans S3 ────────────────────────────────────────────────
    let quote = QuoteRequest {
        id:               Uuid::new_v4().to_string(),
        last_name:        payload.last_name.clone(),
        first_name:       payload.first_name.clone(),
        phone:            payload.phone.clone(),
        email:            payload.email.clone(),
        event_date:       payload.event_date.clone(),
        event_place:      payload.event_place.clone(),
        number_of_people: payload.number_of_people,
        starters:         payload.starters.clone(),
        main_dish:        payload.main_dish.clone(),
        desserts:         payload.desserts.clone(),
        message:          payload.message.clone(),
        created_at:       chrono::Utc::now().to_rfc3339(),
        status:           traiteur_lambdas::QuoteStatus::Pending,
    };

    let mut data: QuotesData = s3_get("data/quotes.json").await;
    data.quotes.push(quote);
    if let Err(e) = s3_put("data/quotes.json", &data).await {
        tracing::warn!("Échec écriture S3 : {e}");
        // On continue — l'email reste prioritaire
    }

    // ── 2. Envoyer l'email via Gmail SMTP ─────────────────────────────────────
    if let Err(e) = send_gmail(&payload).await {
        return Ok(err(500, format!("Envoi email échoué : {e}")));
    }

    Ok(ok(r#"{"success":true}"#))
}

async fn send_gmail(p: &DevisPayload) -> Result<()> {
    let gmail_user = std::env::var("GMAIL_USER")?;
    let gmail_pass = std::env::var("GMAIL_APP_PASSWORD")?;
    let to_email   = std::env::var("CONTACT_EMAIL").unwrap_or_else(|_| gmail_user.clone());

    let body = format!(
        "Nouvelle demande de devis — La Poêlée du Bonheur\n\
         {bar}\n\n\
         Client    : {prenom} {nom}\n\
         Email     : {email}\n\
         Téléphone : {tel}\n\n\
         Événement\n\
         ---------\n\
         Date      : {date}\n\
         Lieu      : {lieu}\n\
         Personnes : {nb}\n\n\
         Menu choisi\n\
         -----------\n\
         Plat principal : {plat}\n\
         Entrées        : {entrees}\n\
         Desserts       : {desserts}\n\n\
         Message\n\
         -------\n\
         {msg}",
        bar      = "=".repeat(50),
        prenom   = p.first_name,
        nom      = p.last_name,
        email    = p.email,
        tel      = if p.phone.is_empty() { "Non renseigné".into() } else { p.phone.clone() },
        date     = if p.event_date.is_empty() { "Non précisée".into() } else { p.event_date.clone() },
        lieu     = if p.event_place.is_empty() { "Non précisé".into() } else { p.event_place.clone() },
        nb       = p.number_of_people,
        plat     = p.main_dish,
        entrees  = if p.starters.is_empty() { "Aucune".into() } else { p.starters.join(", ") },
        desserts = if p.desserts.is_empty() { "Aucun".into() } else { p.desserts.join(", ") },
        msg      = p.message.as_deref().unwrap_or("(aucun)"),
    );

    let subject = format!(
        "🍽️ Devis — {} {} ({} pers.)",
        p.first_name, p.last_name, p.number_of_people
    );

    let from_addr = format!("La Poêlée du Bonheur <{gmail_user}>");
    let reply_to  = p.email.clone();

    let email = Message::builder()
        .from(from_addr.parse()?)
        .reply_to(reply_to.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)?;

    let creds = Credentials::new(gmail_user, gmail_pass);
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(email).await?;
    Ok(())
}
