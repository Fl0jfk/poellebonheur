import { S3Client, GetObjectCommand, PutObjectCommand } from "@aws-sdk/client-s3";
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
const KEY = "data/menu.json";

type MenuItem = {
  id: string;
  name: string;
  description: string;
  photo_url?: string | null;
  category: string;
  price_info?: string | null;
};
type MenuData = { items: MenuItem[] };

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

function menuWithRewrittenPhotos(data: MenuData): MenuData {
  return {
    items: (data.items || []).map((it) => ({
      ...it,
      photo_url: rewriteUploadPhotoUrl(it.photo_url),
    })),
  };
}

function isAdminAuthorized(req: Request): boolean {
  const expected = process.env.ADMIN_API_KEY || "";
  if (!expected) return true;
  return req.headers.get("x-admin-key") === expected;
}

async function signedGet(key: string) {
  return getSignedUrl(
    s3,
    new GetObjectCommand({ Bucket: bucket, Key: key }),
    { expiresIn: 60 },
  );
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

async function saveMenu(data: MenuData): Promise<void> {
  const uploadUrl = await signedPut(KEY, "application/json");
  const put = await fetch(uploadUrl, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(data, null, 2),
  });
  if (!put.ok) throw new Error(`Échec écriture S3 (${put.status})`);
}

async function createImageUploadPresign(contentType: string) {
  const ext = contentType.split("/")[1] || "bin";
  const objectKey = `uploads/${Date.now()}-${Math.random().toString(36).slice(2)}.${ext}`;
  const upload_url = await signedPut(objectKey, contentType);
  return { upload_url, photo_url: publicMediaUrl(objectKey) };
}

function makeId() {
  return `itm_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
}

export async function POST(req: Request) {
  if (!isAdminAuthorized(req)) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }
  const body = (await req.json()) as {
    action?: "list" | "create" | "delete" | "presign_photo";
    id?: string;
    name?: string;
    description?: string;
    photo_url?: string | null;
    category?: string;
    price_info?: string | null;
    content_type?: string;
  };
  const action = body.action || "list";

  if (action === "presign_photo") {
    const contentType = body.content_type || "application/octet-stream";
    if (!contentType.startsWith("image/")) {
      return NextResponse.json({ error: "Image requise" }, { status: 400 });
    }
    return NextResponse.json(await createImageUploadPresign(contentType));
  }

  const menu = await loadMenu();

  if (action === "list") {
    return NextResponse.json(menuWithRewrittenPhotos(menu));
  }

  if (action === "create") {
    if (!body.name?.trim()) {
      return NextResponse.json({ error: "Nom requis" }, { status: 400 });
    }
    const item: MenuItem = {
      id: makeId(),
      name: body.name.trim(),
      description: body.description?.trim() || "",
      category: body.category?.trim() || "starter",
      photo_url: body.photo_url?.trim() || null,
      price_info: null,
    };
    const next = { items: [item, ...(menu.items || [])] };
    await saveMenu(next);
    return NextResponse.json({ ok: true, id: item.id });
  }

  if (action === "delete") {
    if (!body.id) return NextResponse.json({ error: "id requis" }, { status: 400 });
    const next = { items: (menu.items || []).filter((x) => x.id !== body.id) };
    await saveMenu(next);
    return NextResponse.json({ ok: true });
  }

  return NextResponse.json({ error: "Action inconnue" }, { status: 400 });
}
