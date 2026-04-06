import { NextResponse } from "next/server";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

function expectedPassword(): string {
  return (
    process.env.ADMIN_PASSWORD ||
    process.env.NEXT_PUBLIC_ADMIN_PASSWORD ||
    "admin"
  );
}

export async function POST(req: Request) {
  let body: { password?: string };
  try {
    body = (await req.json()) as { password?: string };
  } catch {
    return NextResponse.json({ error: "Corps invalide" }, { status: 400 });
  }
  const pwd = body.password ?? "";
  const expected = expectedPassword();
  if (pwd !== expected) {
    return NextResponse.json({ error: "Mot de passe incorrect" }, { status: 401 });
  }
  return NextResponse.json({ ok: true });
}
