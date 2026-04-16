"use client";

import { useEffect, useMemo, useState } from "react";
import Image from "next/image";

type CollagePhoto = { id: string; src: string; alt: string };

type PhotoCollageProps = {hasMarketBanner?: boolean};

export function PhotoCollage({ hasMarketBanner = false }: PhotoCollageProps) {
  const [loading, setLoading] = useState(true);
  const [photos, setPhotos] = useState<CollagePhoto[]>([]);
  useEffect(() => {
    let ok = true;
    (async () => {
      try {
        const r = await fetch("/api/public/collage", { cache: "no-store" });
        if (!ok) return;
        if (!r.ok) {
          setPhotos([]);
          return;
        }
        const data = (await r.json()) as { photos?: CollagePhoto[] };
        const raw = Array.isArray(data.photos)
          ? data.photos.filter((p) => p?.src && String(p.src).trim())
          : [];
        const list = raw.slice(0, 8);
        setPhotos(list.length >= 5 ? list : []);
      } catch {
        if (ok) setPhotos([]);
      } finally {
        if (ok) setLoading(false);
      }
    })();
    return () => {
      ok = false;
    };
  }, []);
  const [active, setActive] = useState(0);
  useEffect(() => {
    if (photos.length === 0) return;
    setActive((a) => (a >= photos.length ? 0 : a));
  }, [photos.length]);
  useEffect(() => {
    if (photos.length < 2) return;
    const id = window.setInterval(() => {
      setActive((a) => (a + 1) % photos.length);
    }, 3600);
    return () => window.clearInterval(id);
  }, [photos.length]);
  const near = useMemo(() => {
    if (photos.length === 0) return { prev: 0, next: 0 };
    const prev = (active - 1 + photos.length) % photos.length;
    const next = (active + 1) % photos.length;
    return { prev, next };
  }, [active, photos.length]);
  const bannerSpacing = hasMarketBanner ? "max-sm:mt-5 sm:mt-0" : "";
  if (loading) {
    return (
      <div
        className={
          "collage-slider flex min-h-[min(52vw,280px)] w-full items-center justify-center sm:min-h-[clamp(288px,30vw,360px)] " +
          bannerSpacing
        }
      >
        <div
          className="h-11 w-11 animate-spin rounded-full border-4 border-white/40 border-t-white"
          aria-hidden
        />
        <span className="sr-only">Chargement du collage</span>
      </div>
    );
  }
  if (photos.length === 0) {
    return null;
  }
  return (
    <div className={"collage-slider " + bannerSpacing}>
      <div className="collage-stage">
        {photos.map((p, i) => {
          const cls =
            i === active
              ? "pan-card active"
              : i === near.prev
                ? "pan-card prev"
                : i === near.next
                  ? "pan-card next"
                  : "pan-card hidden-card";
          return (
            <button key={p.id} type="button" className={cls} onClick={() => setActive(i)} aria-label={p.alt}>
              <div className="pan-bowl">
                <div className="pan-handle pan-handle-left" aria-hidden />
                <div className="pan-handle pan-handle-right" aria-hidden />
                <div className="pan-inner">
                  <Image src={p.src} alt={p.alt} fill priority={i === active} className="h-full w-full object-cover" />
                </div>
              </div>
            </button>
          );
        })}
        <div className="sr-only" aria-live="polite">
          Photo {active + 1} sur {photos.length}
        </div>
      </div>
    </div>
  );
}
