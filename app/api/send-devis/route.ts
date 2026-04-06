import { GetObjectCommand, PutObjectCommand } from "@aws-sdk/client-s3";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { NextResponse } from "next/server";
import { getStorage, storageConfigErrorJson, type StorageContext } from "@/lib/storage-env";

const KEY = "data/quotes.json";

type QuoteRequest = {
  id: string;
  last_name: string;
  first_name: string;
  phone: string;
  email: string;
  event_date: string;
  event_place: string;
  number_of_people: number;
  starters: string[];
  main_dish: string;
  desserts: string[];
  message?: string | null;
  created_at: string;
};
type QuotesData = { quotes: QuoteRequest[] };

type NewQuote = {
  last_name?: string;
  first_name?: string;
  phone?: string;
  email?: string;
  event_date?: string;
  event_place?: string;
  number_of_people?: number;
  starters?: string[];
  main_dish?: string;
  desserts?: string[];
  message?: string;
};

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

async function saveQuotes(st: StorageContext, data: QuotesData): Promise<void> {
  const uploadUrl = await signedPut(st, KEY, "application/json");
  const put = await fetch(uploadUrl, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(data, null, 2),
  });
  if (!put.ok) throw new Error(`Échec écriture stockage (${put.status})`);
}

function quoteId() {
  return `q_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
}

export async function POST(req: Request) {
  const st = getStorage();
  if (!st) {
    return NextResponse.json(storageConfigErrorJson(), { status: 503 });
  }

  const body = (await req.json()) as NewQuote;
  if (!body.last_name || !body.first_name || !body.email || !body.main_dish) {
    return NextResponse.json({ error: "Champs requis manquants" }, { status: 400 });
  }

  const quotesData = await loadQuotes(st);
  const newQuote: QuoteRequest = {
    id: quoteId(),
    last_name: body.last_name.trim(),
    first_name: body.first_name.trim(),
    phone: body.phone?.trim() || "",
    email: body.email.trim(),
    event_date: body.event_date?.trim() || "",
    event_place: body.event_place?.trim() || "",
    number_of_people: Number(body.number_of_people || 0),
    starters: body.starters || [],
    main_dish: body.main_dish,
    desserts: body.desserts || [],
    message: body.message?.trim() || null,
    created_at: new Date().toISOString(),
  };
  await saveQuotes(st, { quotes: [newQuote, ...(quotesData.quotes || [])] });
  return NextResponse.json({ ok: true, id: newQuote.id });
}
