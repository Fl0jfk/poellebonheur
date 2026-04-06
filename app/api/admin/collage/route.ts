import { GetObjectCommand, PutObjectCommand } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { NextResponse } from "next/server";
import { getStorage, storageConfigErrorJson, type StorageContext } from "@/lib/storage-env";

const KEY = "data/collage.json";

type CollagePhoto = { id: string; src: string; alt: string };
type CollageData = { photos: CollagePhoto[] };

function isAdminAuthorized(req: Request): boolean {
  const expected = process.env.ADMIN_API_KEY;
  if (!expected) return true;
  return req.headers.get("x-admin-key") === expected;
}

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

function publicMediaUrl(objectKey: string) {
  return `/api/public/media?key=${encodeURIComponent(objectKey)}`;
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
      const k = parsed.pathname.slice(1);
      return `/api/public/media?key=${encodeURIComponent(k)}`;
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
        alt: typeof p.alt === "string" && p.alt.trim() ? p.alt.trim() : "Photo traiteur",
      }));
    return { photos };
  } catch {
    return { photos: [] };
  }
}

async function saveCollage(st: StorageContext, data: CollageData): Promise<void> {
  const uploadUrl = await signedPut(st, KEY, "application/json");
  const put = await fetch(uploadUrl, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(data, null, 2),
  });
  if (!put.ok) throw new Error(`Échec écriture stockage (${put.status})`);
}

async function createImageUploadPresign(st: StorageContext, contentType: string) {
  const ext = contentType.split("/")[1] || "bin";
  const objectKey = `uploads/${Date.now()}-${Math.random().toString(36).slice(2)}.${ext}`;
  const upload_url = await signedPut(st, objectKey, contentType);
  return { upload_url, photo_url: publicMediaUrl(objectKey) };
}

export async function POST(req: Request) {
  if (!isAdminAuthorized(req)) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }
  const st = getStorage();
  if (!st) {
    return NextResponse.json(storageConfigErrorJson(), { status: 503 });
  }

  const body = (await req.json()) as {
    action?: "list" | "save" | "presign_photo";
    photos?: CollagePhoto[];
    content_type?: string;
  };
  const action = body.action || "list";

  if (action === "presign_photo") {
    const contentType = body.content_type || "application/octet-stream";
    if (!contentType.startsWith("image/")) {
      return NextResponse.json({ error: "Image requise" }, { status: 400 });
    }
    return NextResponse.json(await createImageUploadPresign(st, contentType));
  }

  if (action === "list") {
    return NextResponse.json(await loadCollage(st));
  }

  if (action === "save") {
    const photos = (body.photos || [])
      .filter((p) => p && typeof p.src === "string" && p.src.trim())
      .map((p, idx) => ({
        id: p.id?.trim() || `cp_${idx}_${Date.now()}`,
        src: p.src.trim(),
        alt: p.alt?.trim() || "Photo traiteur",
      }));
    if (photos.length < 5 || photos.length > 8) {
      return NextResponse.json(
        { error: "Le photocollage doit contenir entre 5 et 8 photos." },
        { status: 400 },
      );
    }
    const invalid = photos.filter((p) => {
      const t = p.src;
      if (t.startsWith("/api/public/media?key=")) return false;
      try {
        const u = new URL(t);
        if (u.hostname.includes("amazonaws.com") && u.pathname.includes("/uploads/")) return false;
      } catch {
        /* ignore */
      }
      return true;
    });
    if (invalid.length > 0) {
      return NextResponse.json(
        {
          error:
            "Chaque photo doit être envoyée via le bouton fichier (URL proxy /api/public/media ou hébergeur objet). Les chemins locaux type /fichier.jpg ne sont pas acceptés.",
        },
        { status: 400 },
      );
    }
    await saveCollage(st, { photos });
    return NextResponse.json({ ok: true });
  }
  return NextResponse.json({ error: "Action inconnue" }, { status: 400 });
}
