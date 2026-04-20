import { GetObjectCommand } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { NextResponse } from "next/server";
import { getStorage, storageConfigErrorJson, type StorageContext } from "@/lib/storage-env";

const KEY = "data/menu.json";

type MenuData = { items: unknown[] };

function rewriteUploadPhotoUrl(
  photoUrl: string | null | undefined,
  bucket: string,
  region: string,
): string | null {
  if (photoUrl == null || !String(photoUrl).trim()) return null;
  const u = String(photoUrl).trim();
  if (u.startsWith("/api/public/media")) return u;
  try {
    const parsed = new URL(u);
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
  return u;
}

async function signedGet(st: StorageContext, key: string) {
  return getSignedUrl(st.s3, new GetObjectCommand({ Bucket: st.bucket, Key: key }), { expiresIn: 60 });
}

async function loadMenu(st: StorageContext): Promise<MenuData> {
  const url = await signedGet(st, KEY);
  const res = await fetch(url, { cache: "no-store" });
  if (res.status === 404 || !res.ok) return { items: [] };
  try {
    const data = (await res.json()) as MenuData;
    return Array.isArray(data.items) ? data : { items: [] };
  } catch {
    return { items: [] };
  }
}

export async function GET() {
  const st = getStorage();
  if (!st) {
    return NextResponse.json(storageConfigErrorJson(), { status: 503 });
  }
  const menu = await loadMenu(st);
  const items = (menu.items || []).map((it) => {
    if (!it || typeof it !== "object") return it;
    const row = it as Record<string, unknown>;
    const photo = row.photo_url;
    if (typeof photo === "string" || photo === null || photo === undefined) {
      return { ...row, photo_url: rewriteUploadPhotoUrl(photo as string | null, st.bucket, st.region) };
    }
    return it;
  });
  return NextResponse.json(
    { items },
    { headers: { "Cache-Control": "public, max-age=300, s-maxage=1800, stale-while-revalidate=86400" } },
  );
}
