import { GetObjectCommand } from "@aws-sdk/client-s3";
import { NextResponse } from "next/server";
import { getStorage, storageConfigErrorJson } from "@/lib/storage-env";

function contentTypeFromKey(key: string): string | null {
  const lower = key.toLowerCase();
  if (lower.endsWith(".avif")) return "image/avif";
  if (lower.endsWith(".webp")) return "image/webp";
  if (lower.endsWith(".jpg") || lower.endsWith(".jpeg")) return "image/jpeg";
  if (lower.endsWith(".png")) return "image/png";
  if (lower.endsWith(".gif")) return "image/gif";
  if (lower.endsWith(".svg")) return "image/svg+xml";
  return null;
}

function safeUploadsKey(raw: string | null): string | null {
  if (!raw) return null;
  let key: string;
  try {
    key = decodeURIComponent(raw);
  } catch {
    return null;
  }
  if (key.includes("..") || key.includes("\\")) return null;
  if (!key.startsWith("uploads/")) return null;
  if (key.length > 512) return null;
  return key;
}

export async function GET(req: Request) {
  const st = getStorage();
  if (!st) {
    return NextResponse.json(storageConfigErrorJson(), { status: 503 });
  }
  const { searchParams } = new URL(req.url);
  const key = safeUploadsKey(searchParams.get("key"));
  if (!key) {
    return new NextResponse("Not found", { status: 404 });
  }
  try {
    const obj = await st.s3.send(
      new GetObjectCommand({
        Bucket: st.bucket,
        Key: key,
      }),
    );
    if (!obj.Body) {
      return new NextResponse("Not found", { status: 404 });
    }
    const buffer = Buffer.from(await obj.Body.transformToByteArray());
    const contentType = obj.ContentType || contentTypeFromKey(key) || "image/jpeg";
    return new NextResponse(buffer, {
      status: 200,
      headers: {
        "Content-Type": contentType,
        "Cache-Control": "public, max-age=31536000, immutable",
      },
    });
  } catch {
    return new NextResponse("Not found", { status: 404 });
  }
}
