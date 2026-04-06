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
const KEY = "data/market.json";

type MarketEntry = { id: string; date: string; place: string };
type MarketsData = { markets: MarketEntry[] };
type LegacyMarket = { date?: string | null; place?: string | null; active?: boolean };

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

function normalizeMarketsData(raw: unknown): MarketsData {
  if (raw && typeof raw === "object" && Array.isArray((raw as MarketsData).markets)) {
    const markets = (raw as MarketsData).markets
      .filter((m) => m && typeof m === "object" && typeof (m as MarketEntry).date === "string")
      .map((m, idx) => {
        const e = m as MarketEntry;
        return {
          id:
            typeof e.id === "string" && e.id
              ? e.id
              : `m_${idx}_${Date.now().toString(36)}_${Math.random().toString(36).slice(2, 6)}`,
          date: e.date,
          place: typeof e.place === "string" ? e.place : "",
        };
      });
    return { markets };
  }
  const legacy = raw as LegacyMarket | null;
  if (legacy && typeof legacy === "object") {
    const d = legacy.date?.trim();
    if (d) {
      return {
        markets: [{ id: "m_legacy", date: d, place: (legacy.place || "").trim() }],
      };
    }
  }
  return { markets: [] };
}

async function loadMarket(): Promise<MarketsData> {
  const url = await signedGet(KEY);
  const res = await fetch(url, { cache: "no-store" });
  if (res.status === 404 || !res.ok) return { markets: [] };
  try {
    const raw = await res.json();
    return normalizeMarketsData(raw);
  } catch {
    return { markets: [] };
  }
}

async function saveMarket(data: MarketsData): Promise<void> {
  const cleaned: MarketsData = {
    markets: (data.markets || [])
      .filter((m) => m.date?.trim())
      .map((m) => ({
        id: m.id?.trim() || `m_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`,
        date: m.date.trim(),
        place: (m.place || "").trim(),
      })),
  };
  const uploadUrl = await signedPut(KEY, "application/json");
  const put = await fetch(uploadUrl, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(cleaned, null, 2),
  });
  if (!put.ok) throw new Error(`Échec écriture S3 (${put.status})`);
}

export async function POST(req: Request) {
  if (!isAdminAuthorized(req)) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }
  const body = (await req.json()) as MarketsData | { markets?: unknown };
  const markets = Array.isArray((body as MarketsData).markets) ? (body as MarketsData).markets : [];
  await saveMarket({ markets });
  return NextResponse.json({ ok: true });
}

export async function GET(req: Request) {
  if (!isAdminAuthorized(req)) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }
  return NextResponse.json(await loadMarket());
}
