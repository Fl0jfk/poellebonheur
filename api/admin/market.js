const { S3Client, PutObjectCommand } = require('@aws-sdk/client-s3');

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

  const { active, date, place } = req.body;

  await s3Client().send(new PutObjectCommand({
    Bucket:      BUCKET,
    Key:         'data/market.json',
    Body:        JSON.stringify({ active: !!active, date: date || null, place: place || null }),
    ContentType: 'application/json',
  }));

  return res.status(200).json({ success: true });
};
