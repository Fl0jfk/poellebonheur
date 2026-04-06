import { S3Client, GetObjectCommand } from "@aws-sdk/client-s3";
import { NextResponse } from "next/server";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

const region = process.env.REGION;
const bucket = process.env.BUCKET_NAME;
if (!bucket) {throw new Error("S3_BUCKET_NAME est requis.")}
const s3 = new S3Client({ region });

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
  const { searchParams } = new URL(req.url);
  const key = safeUploadsKey(searchParams.get("key"));
  if (!key) {
    return new NextResponse("Not found", { status: 404 });
  }
  try {
    const obj = await s3.send(
      new GetObjectCommand({
        Bucket: bucket,
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
