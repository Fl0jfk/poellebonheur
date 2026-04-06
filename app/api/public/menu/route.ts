import { S3Client, GetObjectCommand } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { NextResponse } from "next/server";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

const region = process.env.AWS_REGION || "eu-west-3";
const bucket = process.env.S3_BUCKET_NAME;
if (!bucket) {
  throw new Error("S3_BUCKET_NAME est requis.");
}

const s3 = new S3Client({ region });
const KEY = "data/menu.json";

type MenuData = { items: unknown[] };

function rewriteUploadPhotoUrl(photoUrl: string | null | undefined): string | null {
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

async function signedGet(key: string) {
  return getSignedUrl(
    s3,
    new GetObjectCommand({ Bucket: bucket, Key: key }),
    { expiresIn: 60 },
  );
}

async function loadMenu(): Promise<MenuData> {
  const url = await signedGet(KEY);
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
  const menu = await loadMenu();
  const items = (menu.items || []).map((it) => {
    if (!it || typeof it !== "object") return it;
    const row = it as Record<string, unknown>;
    const photo = row.photo_url;
    if (typeof photo === "string" || photo === null || photo === undefined) {
      return { ...row, photo_url: rewriteUploadPhotoUrl(photo as string | null) };
    }
    return it;
  });
  return NextResponse.json({ items });
}
