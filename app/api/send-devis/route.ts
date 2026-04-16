import { GetObjectCommand, PutObjectCommand } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { NextResponse } from "next/server";
import nodemailer from "nodemailer";
import { getStorage, storageConfigErrorJson, type StorageContext } from "@/lib/storage-env";

const KEY = "data/quotes.json";

type QuoteRequest = {
  id: string;
  last_name: string;
  first_name: string;
  phone: string;
  email: string;
  event_date: string;
  event_place: string;
  number_of_people: number;
  starters: string[];
  starters_labels?: string[];
  main_dish: string;
  main_dish_label?: string;
  desserts: string[];
  desserts_labels?: string[];
  message?: string | null;
  created_at: string;
};
type QuotesData = { quotes: QuoteRequest[] };

type NewQuote = {
  last_name?: string;
  first_name?: string;
  phone?: string;
  email?: string;
  event_date?: string;
  event_place?: string;
  number_of_people?: number;
  starters?: string[];
  starters_labels?: string[];
  main_dish?: string;
  main_dish_label?: string;
  desserts?: string[];
  desserts_labels?: string[];
  message?: string;
};

async function signedGet(st: StorageContext, key: string) {
  return getSignedUrl(st.s3, new GetObjectCommand({ Bucket: st.bucket, Key: key }), { expiresIn: 60 });
}

async function signedPut(st: StorageContext, key: string, contentType: string) {
  return getSignedUrl(
    st.s3,
    new PutObjectCommand({ Bucket: st.bucket, Key: key, ContentType: contentType }),
    { expiresIn: 60 },
  );
}

async function loadQuotes(st: StorageContext): Promise<QuotesData> {
  const url = await signedGet(st, KEY);
  const res = await fetch(url, { cache: "no-store" });
  if (res.status === 404 || !res.ok) return { quotes: [] };
  try {
    const data = (await res.json()) as QuotesData;
    return Array.isArray(data.quotes) ? data : { quotes: [] };
  } catch {
    return { quotes: [] };
  }
}

async function saveQuotes(st: StorageContext, data: QuotesData): Promise<void> {
  const uploadUrl = await signedPut(st, KEY, "application/json");
  const put = await fetch(uploadUrl, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(data, null, 2),
  });
  if (!put.ok) throw new Error(`Échec écriture stockage (${put.status})`);
}

function quoteId() {
  return `q_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
}

function formatList(values: string[]): string {
  return values.filter(Boolean).join(", ") || "Aucun";
}

function escapeHtml(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

function createMailer() {
  const user = process.env.SMTP_USER?.trim();
  const pass = process.env.SMTP_PASS?.trim();
  if (!user || !pass) {
    throw new Error("Configuration e-mail manquante : ajoutez SMTP_USER et SMTP_PASS.");
  }
  return {
    user,
    transporter: nodemailer.createTransport({
      host: "smtp.gmail.com",
      port: 465,
      secure: true,
      auth: { user, pass },
    }),
  };
}

async function sendQuoteEmails(quote: QuoteRequest) {
  const { user, transporter } = createMailer();
  const customerName = `${quote.first_name} ${quote.last_name}`.trim();
  const starters = formatList(quote.starters_labels || []);
  const mainDish = quote.main_dish_label?.trim() || quote.main_dish;
  const desserts = formatList(quote.desserts_labels || []);
  const eventDate = quote.event_date || "Non précisée";
  const eventPlace = quote.event_place || "Non précisé";
  const message = quote.message?.trim() || "Aucun message complémentaire.";
  const customerNameHtml = escapeHtml(customerName);
  const emailHtml = escapeHtml(quote.email);
  const phoneHtml = escapeHtml(quote.phone || "Non renseigné");
  const dateHtml = escapeHtml(eventDate);
  const placeHtml = escapeHtml(eventPlace);
  const startersHtml = escapeHtml(starters);
  const mainDishHtml = escapeHtml(mainDish || "Non renseigné");
  const dessertsHtml = escapeHtml(desserts);
  const messageHtml = escapeHtml(message).replace(/\n/g, "<br />");

  const summaryText = [
    `Nom : ${customerName}`,
    `Email : ${quote.email}`,
    `Téléphone : ${quote.phone || "Non renseigné"}`,
    `Date : ${eventDate}`,
    `Lieu : ${eventPlace}`,
    `Nombre de personnes : ${quote.number_of_people || 0}`,
    `Entrées : ${starters}`,
    `Plat principal : ${mainDish || "Non renseigné"}`,
    `Desserts : ${desserts}`,
    `Message : ${message}`,
  ].join("\n");

  await Promise.all([
    transporter.sendMail({
      from: `"La Poêlée du Bonheur" <${user}>`,
      to: user,
      replyTo: quote.email,
      subject: `Nouvelle demande de devis - ${customerName}`,
      text: `Une nouvelle demande de devis a été reçue.\n\n${summaryText}`,
      html: `
        <h2>Nouvelle demande de devis</h2>
        <p><strong>Nom :</strong> ${customerNameHtml}</p>
        <p><strong>Email :</strong> ${emailHtml}</p>
        <p><strong>Téléphone :</strong> ${phoneHtml}</p>
        <p><strong>Date :</strong> ${dateHtml}</p>
        <p><strong>Lieu :</strong> ${placeHtml}</p>
        <p><strong>Nombre de personnes :</strong> ${quote.number_of_people || 0}</p>
        <p><strong>Entrées :</strong> ${startersHtml}</p>
        <p><strong>Plat principal :</strong> ${mainDishHtml}</p>
        <p><strong>Desserts :</strong> ${dessertsHtml}</p>
        <p><strong>Message :</strong><br />${messageHtml}</p>
      `,
    }),
    transporter.sendMail({
      from: `"La Poêlée du Bonheur" <${user}>`,
      to: quote.email,
      subject: "Votre demande de devis a bien été reçue",
      text:
        `Bonjour ${quote.first_name},\n\n` +
        "Merci pour votre demande de devis auprès de La Poêlée du Bonheur.\n" +
        "Nous avons bien reçu votre demande et nous reviendrons vers vous rapidement.\n\n" +
        "Récapitulatif :\n" +
        `${summaryText}\n\n` +
        "A bientôt,\nLa Poêlée du Bonheur",
      html: `
        <p>Bonjour ${escapeHtml(quote.first_name)},</p>
        <p>Merci pour votre demande de devis auprès de <strong>La Poêlée du Bonheur</strong>.</p>
        <p>Nous avons bien reçu votre demande et nous reviendrons vers vous rapidement.</p>
        <h3>Récapitulatif</h3>
        <p><strong>Date :</strong> ${dateHtml}</p>
        <p><strong>Lieu :</strong> ${placeHtml}</p>
        <p><strong>Nombre de personnes :</strong> ${quote.number_of_people || 0}</p>
        <p><strong>Entrées :</strong> ${startersHtml}</p>
        <p><strong>Plat principal :</strong> ${mainDishHtml}</p>
        <p><strong>Desserts :</strong> ${dessertsHtml}</p>
        <p><strong>Message :</strong><br />${messageHtml}</p>
        <p>A bientôt,<br /><strong>La Poêlée du Bonheur</strong></p>
      `,
    }),
  ]);
}

export async function POST(req: Request) {
  const st = getStorage();
  if (!st) {
    return NextResponse.json(storageConfigErrorJson(), { status: 503 });
  }

  const body = (await req.json()) as NewQuote;
  if (!body.last_name || !body.first_name || !body.email || !body.main_dish) {
    return NextResponse.json({ error: "Champs requis manquants" }, { status: 400 });
  }

  const quotesData = await loadQuotes(st);
  const newQuote: QuoteRequest = {
    id: quoteId(),
    last_name: body.last_name.trim(),
    first_name: body.first_name.trim(),
    phone: body.phone?.trim() || "",
    email: body.email.trim(),
    event_date: body.event_date?.trim() || "",
    event_place: body.event_place?.trim() || "",
    number_of_people: Number(body.number_of_people || 0),
    starters: body.starters || [],
    starters_labels: Array.isArray(body.starters_labels)
      ? body.starters_labels.filter((x): x is string => typeof x === "string" && x.trim().length > 0)
      : [],
    main_dish: body.main_dish,
    main_dish_label:
      typeof body.main_dish_label === "string" && body.main_dish_label.trim()
        ? body.main_dish_label.trim()
        : "",
    desserts: body.desserts || [],
    desserts_labels: Array.isArray(body.desserts_labels)
      ? body.desserts_labels.filter((x): x is string => typeof x === "string" && x.trim().length > 0)
      : [],
    message: body.message?.trim() || null,
    created_at: new Date().toISOString(),
  };
  await saveQuotes(st, { quotes: [newQuote, ...(quotesData.quotes || [])] });
  await sendQuoteEmails(newQuote);
  return NextResponse.json({ ok: true, id: newQuote.id });
}
