import { GetObjectCommand } from "@aws-sdk/client-s3";
import { NextResponse } from "next/server";
import { getStorage, storageConfigErrorJson } from "@/lib/storage-env";

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
    const contentType = obj.ContentType || "application/octet-stream";
    return new NextResponse(buffer, {
      status: 200,
      headers: {
        "Content-Type": contentType,
        "Cache-Control": "public, max-age=86400, stale-while-revalidate=604800",
      },
    });
  } catch {
    return new NextResponse("Not found", { status: 404 });
  }
}
