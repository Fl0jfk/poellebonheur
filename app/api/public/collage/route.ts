import { GetObjectCommand } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { NextResponse } from "next/server";
import { getStorage, storageConfigErrorJson, type StorageContext } from "@/lib/storage-env";

const KEY = "data/collage.json";

type CollagePhoto = { id: string; src: string; alt: string };
type CollageData = { photos: CollagePhoto[] };

async function signedGet(st: StorageContext, key: string) {
  return getSignedUrl(st.s3, new GetObjectCommand({ Bucket: st.bucket, Key: key }), { expiresIn: 60 });
}

function rewriteUploadSrc(url: string, bucket: string, region: string): string {
  const t = url.trim();
  if (t.startsWith("/api/public/media")) return t;
  try {
    const parsed = new URL(t);
    if (
      parsed.hostname === `${bucket}.s3.${region}.amazonaws.com` &&
      parsed.pathname.startsWith("/uploads/")
    ) {
      const key = parsed.pathname.slice(1);
      return `/api/public/media?key=${encodeURIComponent(key)}`;
    }
  } catch {
    /* ignore */
  }
  return t;
}

async function loadCollage(st: StorageContext): Promise<CollageData> {
  const url = await signedGet(st, KEY);
  const res = await fetch(url, { cache: "no-store" });
  if (res.status === 404 || !res.ok) return { photos: [] };
  try {
    const raw = (await res.json()) as { photos?: unknown };
    if (!Array.isArray(raw.photos)) return { photos: [] };
    const photos = raw.photos
      .filter(
        (p): p is CollagePhoto =>
          p != null &&
          typeof p === "object" &&
          typeof (p as CollagePhoto).src === "string" &&
          Boolean((p as CollagePhoto).src.trim()),
      )
      .map((p, idx) => ({
        id: typeof p.id === "string" && p.id ? p.id : `cp_${idx}`,
        src: rewriteUploadSrc(p.src.trim(), st.bucket, st.region),
        alt: typeof p.alt === "string" && p.alt.trim() ? p.alt.trim() : "Photo",
      }));
    return { photos };
  } catch {
    return { photos: [] };
  }
}

export async function GET() {
  const st = getStorage();
  if (!st) {
    return NextResponse.json(storageConfigErrorJson(), { status: 503 });
  }
  const collage = await loadCollage(st);
  return NextResponse.json(collage);
}
