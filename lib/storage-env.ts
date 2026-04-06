import { S3Client } from "@aws-sdk/client-s3";

export type StorageContext = {
  s3: S3Client;
  bucket: string;
  region: string;
};

let cache: StorageContext | null = null;
let cacheKey = "";

function resolveBucket(): string {
  return (
    process.env.BUCKET_NAME ||
    process.env.STORAGE_BUCKET ||
    process.env.S3_BUCKET_NAME ||
    ""
  ).trim();
}

function resolveRegion(): string {
  return (
    process.env.REGION ||
    process.env.STORAGE_REGION ||
    process.env.AWS_REGION ||
    "eu-west-3"
  ).trim();
}

/**
 * Contexte S3 pour les routes API. À appeler dans le handler (pas au chargement du module) :
 * sur Amplify, les variables ne sont pas toujours présentes tant que le module n’est pas exécuté.
 */
export function getStorage(): StorageContext | null {
  const bucket = resolveBucket();
  if (!bucket) return null;
  const region = resolveRegion();
  const accessKeyId = process.env.ACCESS_KEY_ID || process.env.AWS_ACCESS_KEY_ID;
  const secretAccessKey = process.env.SECRET_ACCESS_KEY || process.env.AWS_SECRET_ACCESS_KEY;
  const key = `${bucket}\0${region}\0${accessKeyId || ""}`;
  if (cache && cacheKey === key) return cache;
  const s3 =
    accessKeyId && secretAccessKey
      ? new S3Client({ region, credentials: { accessKeyId, secretAccessKey } })
      : new S3Client({ region });
  cache = { s3, bucket, region };
  cacheKey = key;
  return cache;
}

/** Corps JSON pour réponse 503 si BUCKET_NAME manque côté serveur. */
export function storageConfigErrorJson() {
  return {
    error:
      "Stockage non configuré : ajoutez BUCKET_NAME (et idéalement REGION) dans Amplify → Hosting → Environment variables, en cochant bien l’option pour le **runtime** / SSR, pas seulement le build.",
  };
}
