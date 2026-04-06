import { PutObjectCommand } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { NextResponse } from "next/server";
import { getStorage, storageConfigErrorJson, type StorageContext } from "@/lib/storage-env";

function isAdminAuthorized(req: Request): boolean {
  const expected = process.env.ADMIN_API_KEY || "";
  if (!expected) return true;
  return req.headers.get("x-admin-key") === expected;
}

async function signedPut(st: StorageContext, key: string, contentType: string) {
  return getSignedUrl(
    st.s3,
    new PutObjectCommand({ Bucket: st.bucket, Key: key, ContentType: contentType }),
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
  const st = getStorage();
  if (!st) {
    return NextResponse.json(storageConfigErrorJson(), { status: 503 });
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
  const upload_url = await signedPut(st, objectKey, file.type);
  const put = await fetch(upload_url, {
    method: "PUT",
    headers: { "Content-Type": file.type },
    body: file,
  });
  if (!put.ok) {
    return NextResponse.json({ error: "Upload refusé par le stockage" }, { status: 502 });
  }
  return NextResponse.json({ photo_url: publicMediaUrl(objectKey) });
}
