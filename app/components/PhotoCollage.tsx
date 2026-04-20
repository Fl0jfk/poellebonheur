"use client";

import { useEffect, useMemo, useState } from "react";
import Image from "next/image";
import { getBlurDataURL } from "@/app/lib/image-placeholder";

type CollagePhoto = { id: string; src: string; alt: string };

type PhotoCollageProps = {hasMarketBanner?: boolean};

export function PhotoCollage({ hasMarketBanner = false }: PhotoCollageProps) {
  const blurDataURL = useMemo(() => getBlurDataURL("#dec7a5"), []);
  const isProxiedMedia = (src: string) => src.startsWith("/api/public/media?");
  const [loading, setLoading] = useState(true);
  const [photos, setPhotos] = useState<CollagePhoto[]>([]);
  const [instantPhoto, setInstantPhoto] = useState<CollagePhoto | null>(null);
  useEffect(() => {
    if (typeof window === "undefined") return;
    try {
      const cached = window.localStorage.getItem("hero-collage-first-photo");
      if (!cached) return;
      const parsed = JSON.parse(cached) as CollagePhoto;
      if (parsed?.src) setInstantPhoto(parsed);
    } catch {
      // Ignore invalid cache payloads.
    }
  }, []);
  useEffect(() => {
    let ok = true;
    (async () => {
      try {
        const r = await fetch("/api/public/collage", { cache: "force-cache" });
        if (!ok) return;
        if (!r.ok) {
          setPhotos([]);
          return;
        }
        const data = (await r.json()) as { photos?: CollagePhoto[] };
        const raw = Array.isArray(data.photos) ? data.photos.filter((p) => p?.src && String(p.src).trim()) : [];
        const list = raw.slice(0, 8);
        setPhotos(list.length >= 5 ? list : []);
        if (list[0]?.src) {
          setInstantPhoto(list[0]);
          if (typeof window !== "undefined") { window.localStorage.setItem("hero-collage-first-photo", JSON.stringify(list[0]))}
        }
      } catch {
        if (ok) setPhotos([]);
      } finally {
        if (ok) setLoading(false);
      }
    })();
    return () => { ok = false};
  }, []);
  const [active, setActive] = useState(0);
  useEffect(() => {
    if (photos.length === 0) return;
    setActive((a) => (a >= photos.length ? 0 : a));
  }, [photos.length]);
  useEffect(() => {
    if (photos.length < 2) return;
    const id = window.setInterval(() => { setActive((a) => (a + 1) % photos.length)}, 3600);
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
    if (instantPhoto) {
      return (
        <div className={"collage-slider " + bannerSpacing}>
          <div className="collage-stage">
            <div className="pan-card active">
              <div className="pan-bowl">
                <div className="pan-handle pan-handle-left" aria-hidden />
                <div className="pan-handle pan-handle-right" aria-hidden />
                <div className="pan-inner">
                  <Image
                    src={instantPhoto.src}
                    alt={instantPhoto.alt || "Photo de présentation"}
                    fill
                    priority
                    unoptimized={isProxiedMedia(instantPhoto.src)}
                    quality={30}
                    sizes="(max-width: 768px) 88vw, 720px"
                    placeholder="blur"
                    blurDataURL={blurDataURL}
                    className="h-full w-full scale-[1.03] object-cover blur-[3px]"
                  />
                </div>
              </div>
            </div>
            <div className="sr-only" aria-live="polite">Aperçu photo en chargement</div>
          </div>
        </div>
      );
    }
    return (
      <div className={"collage-slider flex min-h-[min(52vw,280px)] w-full items-center justify-center sm:min-h-[clamp(288px,30vw,360px)] " + bannerSpacing}>
        <div className="h-11 w-11 animate-spin rounded-full border-4 border-white/40 border-t-white" aria-hidden/>
        <span className="sr-only">Chargement du collage</span>
      </div>
    );
  }
  if (photos.length === 0) { return null}
  return (
    <div className={"collage-slider " + bannerSpacing}>
      <div className="collage-stage">
        {photos.map((p, i) => {
          const cls = i === active ? "pan-card active" : i === near.prev ? "pan-card prev" : i === near.next ? "pan-card next" : "pan-card hidden-card";
          return (
            <button key={p.id} type="button" className={cls} onClick={() => setActive(i)} aria-label={p.alt}>
              <div className="pan-bowl">
                <div className="pan-handle pan-handle-left" aria-hidden />
                <div className="pan-handle pan-handle-right" aria-hidden />
                <div className="pan-inner">
                  <Image src={p.src} alt={p.alt} fill priority={i === active} unoptimized={isProxiedMedia(p.src)} quality={70} sizes="(max-width: 768px) 88vw, 720px" placeholder="blur" blurDataURL={blurDataURL} className="h-full w-full object-cover"/>
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
