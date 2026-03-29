const nodemailer = require('nodemailer');
const { S3Client, GetObjectCommand, PutObjectCommand } = require('@aws-sdk/client-s3');

// ── helpers S3 ────────────────────────────────────────────────────────────────

function s3Client() {
  return new S3Client({
    region: process.env.AWS_REGION || 'eu-west-3',
    credentials: {
      accessKeyId:     process.env.ACCESS_KEY_ID,
      secretAccessKey: process.env.SECRET_ACCESS_KEY,
    },
  });
}

const BUCKET = process.env.S3_BUCKET_NAME || 'poellebonheur';

async function s3Get(key, fallback) {
  try {
    const resp = await s3Client().send(new GetObjectCommand({ Bucket: BUCKET, Key: key }));
    const chunks = [];
    for await (const chunk of resp.Body) chunks.push(chunk);
    return JSON.parse(Buffer.concat(chunks).toString('utf-8'));
  } catch {
    return fallback;
  }
}

async function s3Put(key, data) {
  await s3Client().send(new PutObjectCommand({
    Bucket:      BUCKET,
    Key:         key,
    Body:        JSON.stringify(data),
    ContentType: 'application/json',
  }));
}

// ── handler ───────────────────────────────────────────────────────────────────

module.exports = async (req, res) => {
  res.setHeader('Access-Control-Allow-Origin',  '*');
  res.setHeader('Access-Control-Allow-Methods', 'POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
  if (req.method === 'OPTIONS') return res.status(200).end();
  if (req.method !== 'POST')   return res.status(405).json({ error: 'Method not allowed' });

  const {
    last_name, first_name, phone, email,
    event_date, event_place, number_of_people,
    starters, main_dish, desserts, message,
  } = req.body;

  // Validation minimale
  if (!last_name || !first_name || !email || !main_dish || !number_of_people) {
    return res.status(400).json({ error: 'Champs obligatoires manquants' });
  }

  // ── 1. Stocker le devis dans S3 ────────────────────────────────────────────
  try {
    const data = await s3Get('data/quotes.json', { quotes: [] });
    data.quotes.push({
      id:               crypto.randomUUID(),
      last_name, first_name, phone: phone || '', email,
      event_date:       event_date || '',
      event_place:      event_place || '',
      number_of_people: Number(number_of_people),
      starters:         starters || [],
      main_dish,
      desserts:         desserts || [],
      message:          message || null,
      created_at:       new Date().toISOString(),
      status:           'Pending',
    });
    await s3Put('data/quotes.json', data);
  } catch (err) {
    console.error('S3 write error:', err);
    // On continue même si S3 échoue — l'email reste prioritaire
  }

  // ── 2. Envoyer l'email via Gmail ───────────────────────────────────────────
  const transporter = nodemailer.createTransport({
    service: 'gmail',
    auth: {
      user: process.env.GMAIL_USER,
      pass: process.env.GMAIL_APP_PASSWORD,
    },
  });

  const body = [
    `Nouvelle demande de devis — La Poêlée du Bonheur`,
    `${'='.repeat(50)}`,
    ``,
    `Client    : ${first_name} ${last_name}`,
    `Email     : ${email}`,
    `Téléphone : ${phone || 'Non renseigné'}`,
    ``,
    `Événement`,
    `---------`,
    `Date      : ${event_date || 'Non précisée'}`,
    `Lieu      : ${event_place || 'Non précisé'}`,
    `Personnes : ${number_of_people}`,
    ``,
    `Menu choisi`,
    `-----------`,
    `Plat principal : ${main_dish}`,
    `Entrées        : ${(starters || []).join(', ') || 'Aucune'}`,
    `Desserts       : ${(desserts || []).join(', ') || 'Aucun'}`,
    ``,
    `Message complémentaire`,
    `----------------------`,
    message || '(aucun)',
  ].join('\n');

  await transporter.sendMail({
    from:    `"La Poêlée du Bonheur" <${process.env.GMAIL_USER}>`,
    to:      process.env.CONTACT_EMAIL || process.env.GMAIL_USER,
    replyTo: email,
    subject: `🍽️ Devis — ${first_name} ${last_name} (${number_of_people} pers.)`,
    text:    body,
  });

  return res.status(200).json({ success: true });
};
