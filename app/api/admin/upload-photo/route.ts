import { S3Client, PutObjectCommand } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { NextResponse } from "next/server";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

const region = process.env.REGION;
const bucket = process.env.BUCKET_NAME;
if (!bucket) {
  throw new Error("S3_BUCKET_NAME est requis.");
}

const s3 = new S3Client({ region });

function isAdminAuthorized(req: Request): boolean {
  const expected = process.env.ADMIN_API_KEY  || "";
  if (!expected) return true;
  return req.headers.get("x-admin-key") === expected;
}

async function signedPut(key: string, contentType: string) {
  return getSignedUrl(
    s3,
    new PutObjectCommand({ Bucket: bucket, Key: key, ContentType: contentType }),
    { expiresIn: 60 },
  );
}

function publicMediaUrl(objectKey: string) {
  return `/api/public/media?key=${encodeURIComponent(objectKey)}`;
}

export async function POST(req: Request) {
  if (!isAdminAuthorized(req)) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }
  const formData = await req.formData();
  const file = formData.get("file");
  if (!(file instanceof File)) {
    return NextResponse.json({ error: "Fichier manquant" }, { status: 400 });
  }
  if (!file.type.startsWith("image/")) {
    return NextResponse.json({ error: "Image requise" }, { status: 400 });
  }
  const ext = file.type.split("/")[1] || "bin";
  const objectKey = `uploads/${Date.now()}-${Math.random().toString(36).slice(2)}.${ext}`;
  const upload_url = await signedPut(objectKey, file.type);
  const put = await fetch(upload_url, {
    method: "PUT",
    headers: { "Content-Type": file.type },
    body: file,
  });
  if (!put.ok) {
    return NextResponse.json({ error: "Upload S3 refusé" }, { status: 502 });
  }
  return NextResponse.json({ photo_url: publicMediaUrl(objectKey) });
}
