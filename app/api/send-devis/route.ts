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
  return value.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;").replaceAll('"', "&quot;").replaceAll("'", "&#39;");
}

function createEmailLayout(params: {
  title: string;
  intro: string;
  infoRowsHtml: string;
  menuRowsHtml: string;
  messageHtml: string;
}) {
  const logoUrl = process.env.NEXT_PUBLIC_SITE_URL?.trim()
    ? `${process.env.NEXT_PUBLIC_SITE_URL.trim().replace(/\/$/, "")}/Logo.png`
    : "";
  const safeLogoUrl = escapeHtml(logoUrl);
  return `
    <div style="margin:0; padding:0; background:#fff7f0; font-family: Arial, Helvetica, sans-serif; color:#2c1c0f;">
      <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="padding:24px 10px;">
        <tr>
          <td align="center">
            <table role="presentation" cellpadding="0" cellspacing="0" width="640" style="width:100%; max-width:640px; background:#ffffff; border-radius:16px; border:1px solid #f3dcc8; overflow:hidden;">
              <tr>
                <td style="padding:24px; background:linear-gradient(135deg, #ffedd5, #ffe4cc); border-bottom:1px solid #f3dcc8;">
                  ${
                    safeLogoUrl
                      ? `<img src="${safeLogoUrl}" alt="Logo La Poêlée du Bonheur" width="120" style="display:block; margin:0 auto 12px auto;" />`
                      : ""
                  }
                  <p style="margin:0; text-align:center; color:#92400e; font-size:13px; letter-spacing:0.04em; text-transform:uppercase;">La Poêlée du Bonheur</p>
                  <h1 style="margin:10px 0 0 0; text-align:center; font-size:24px; line-height:1.3; color:#7c2d12;">${escapeHtml(params.title)}</h1>
                </td>
              </tr>
              <tr>
                <td style="padding:24px;">
                  <p style="margin:0 0 18px 0; font-size:15px; line-height:1.6; color:#3f2a17;">${escapeHtml(params.intro)}</p>
                  <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="margin-bottom:16px; background:#fffbf8; border:1px solid #f6e5d6; border-radius:12px;">
                    <tr>
                      <td style="padding:14px 16px;">
                        <p style="margin:0 0 10px 0; font-size:14px; font-weight:bold; color:#9a3412;">Informations</p>
                        ${params.infoRowsHtml}
                      </td>
                    </tr>
                  </table>
                  <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="margin-bottom:16px; background:#fffbf8; border:1px solid #f6e5d6; border-radius:12px;">
                    <tr>
                      <td style="padding:14px 16px;">
                        <p style="margin:0 0 10px 0; font-size:14px; font-weight:bold; color:#9a3412;">Repas choisi</p>
                        ${params.menuRowsHtml}
                      </td>
                    </tr>
                  </table>
                  <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="background:#fffbf8; border:1px solid #f6e5d6; border-radius:12px;">
                    <tr>
                      <td style="padding:14px 16px;">
                        <p style="margin:0 0 10px 0; font-size:14px; font-weight:bold; color:#9a3412;">Message</p>
                        <p style="margin:0; font-size:14px; line-height:1.6; color:#3f2a17;">${params.messageHtml}</p>
                      </td>
                    </tr>
                  </table>
                </td>
              </tr>
              <tr>
                <td style="padding:18px 24px; border-top:1px solid #f3dcc8; background:#fff7f0;">
                  <p style="margin:0 0 6px 0; color:#5f3e1f; font-size:13px;">La Poêlée du Bonheur</p>
                </td>
              </tr>
            </table>
          </td>
        </tr>
      </table>
    </div>
  `;
}

function createInfoRow(icon: string, label: string, value: string): string {
  return `<p style="margin:0 0 8px 0; font-size:14px; line-height:1.5;"><span style="font-size:14px; margin-right:6px;">${icon}</span><strong>${escapeHtml(label)} :</strong> ${escapeHtml(value)}</p>`;
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
  const messageHtml = escapeHtml(message).replace(/\n/g, "<br />");
  const createdAt = new Date(quote.created_at).toLocaleString("fr-FR", {
    dateStyle: "medium",
    timeStyle: "short",
  });

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

  const infoRowsHtml =
    createInfoRow("👤", "Nom", customerName || "Non renseigné") +
    createInfoRow("✉️", "Email", quote.email) +
    createInfoRow("📞", "Téléphone", quote.phone || "Non renseigné") +
    createInfoRow("📅", "Date de l'événement", eventDate) +
    createInfoRow("📍", "Lieu", eventPlace) +
    createInfoRow("👥", "Nombre de personnes", `${quote.number_of_people || 0}`) +
    createInfoRow("🕒", "Demande reçue le", createdAt);

  const menuRowsHtml =
    createInfoRow("🥗", "Entrées", starters) +
    createInfoRow("🍽️", "Plat principal", mainDish || "Non renseigné") +
    createInfoRow("🍰", "Desserts", desserts);

  const adminHtml = createEmailLayout({
    title: "Nouvelle demande de devis",
    intro: "Une nouvelle demande de devis vient d'être envoyée via le site.",
    infoRowsHtml,
    menuRowsHtml,
    messageHtml
  });

  const customerHtml = createEmailLayout({
    title: "Votre demande de devis a bien été reçue",
    intro: `Bonjour ${quote.first_name}, votre demande est enregistrée. Retrouvez ci-dessous toutes les informations transmises.`,
    infoRowsHtml,
    menuRowsHtml,
    messageHtml
  });

  await Promise.all([
    transporter.sendMail({
      from: `"La Poêlée du Bonheur" <${user}>`,
      to: user,
      replyTo: quote.email,
      subject: `Nouvelle demande de devis - ${customerName}`,
      text: `Une nouvelle demande de devis a été reçue.\n\n${summaryText}`,
      html: adminHtml,
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
      html: customerHtml,
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
