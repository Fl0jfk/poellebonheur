import { GetObjectCommand } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { NextResponse } from "next/server";
import { getStorage, storageConfigErrorJson, type StorageContext } from "@/lib/storage-env";

const KEY = "data/quotes.json";

type QuotesData = { quotes: unknown[] };

function isAdminAuthorized(req: Request): boolean {
  const expected = process.env.ADMIN_API_KEY || "";
  if (!expected) return true;
  return req.headers.get("x-admin-key") === expected;
}

async function signedGet(st: StorageContext, key: string) {
  return getSignedUrl(st.s3, new GetObjectCommand({ Bucket: st.bucket, Key: key }), { expiresIn: 60 });
}

async function loadQuotes(st: StorageContext): Promise<QuotesData> {
  const url = await signedGet(st, KEY);
  const res = await fetch(url, { cache: "no-store" });
  if (res.status === 404 || !res.ok) return { quotes: [] };
  try {
    const data = (await res.json()) as QuotesData;
    return Array.isArray(data.quotes) ? data : { quotes: [] };
  } catch {
    return { quotes: [] };
  }
}

export async function GET(req: Request) {
  if (!isAdminAuthorized(req)) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }
  const st = getStorage();
  if (!st) {
    return NextResponse.json(storageConfigErrorJson(), { status: 503 });
  }
  return NextResponse.json(await loadQuotes(st));
}
