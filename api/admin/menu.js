const { S3Client, GetObjectCommand, PutObjectCommand } = require('@aws-sdk/client-s3');
const { randomUUID } = require('crypto');

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

async function getMenu() {
  try {
    const resp = await s3Client().send(new GetObjectCommand({ Bucket: BUCKET, Key: 'data/menu.json' }));
    const chunks = [];
    for await (const chunk of resp.Body) chunks.push(chunk);
    return JSON.parse(Buffer.concat(chunks).toString('utf-8'));
  } catch {
    return { items: [] };
  }
}

async function putMenu(data) {
  await s3Client().send(new PutObjectCommand({
    Bucket:      BUCKET,
    Key:         'data/menu.json',
    Body:        JSON.stringify(data),
    ContentType: 'application/json',
  }));
}

module.exports = async (req, res) => {
  res.setHeader('Access-Control-Allow-Origin',  '*');
  res.setHeader('Access-Control-Allow-Methods', 'POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type, x-admin-key');
  if (req.method === 'OPTIONS') return res.status(200).end();
  if (req.method !== 'POST')   return res.status(405).json({ error: 'Method not allowed' });

  const adminKey = process.env.ADMIN_API_KEY;
  if (adminKey && req.headers['x-admin-key'] !== adminKey) {
    return res.status(401).json({ error: 'Non autorisé' });
  }

  const { action, id, name, description, category, price_info } = req.body;

  const menu = await getMenu();

  if (action === 'create') {
    if (!name || !name.trim()) {
      return res.status(400).json({ error: 'Le nom est obligatoire' });
    }
    const item = {
      id:          randomUUID(),
      name:        name.trim(),
      description: description || null,
      photo_url:   null,
      category:    category || 'MainDish',
      price_info:  price_info || null,
    };
    menu.items.push(item);
    await putMenu(menu);
    return res.status(200).json({ success: true, item });
  }

  if (action === 'delete') {
    if (!id) return res.status(400).json({ error: 'id requis' });
    const before = menu.items.length;
    menu.items = menu.items.filter(i => i.id !== id);
    if (menu.items.length === before) {
      return res.status(404).json({ error: 'Plat introuvable' });
    }
    await putMenu(menu);
    return res.status(200).json({ success: true });
  }

  return res.status(400).json({ error: 'action inconnue' });
};
