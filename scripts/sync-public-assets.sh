#!/usr/bin/env bash
# Après `dx build` ou `dx serve` (premier build), réinjecte les images depuis public/
# dans la sortie web, car dioxus-cli peut altérer les .avif lors du pipeline « image ».
set -euo pipefail
PUBLIC_DIR="$(find target/dx -type d -path '*/release/web/public' -o -path '*/debug/web/public' 2>/dev/null | head -n 1)"
if [ -z "$PUBLIC_DIR" ]; then
  echo "Dossier target/dx/.../web/public introuvable — lance d’abord dx build --platform web" >&2
  exit 1
fi
shopt -s nullglob
for f in public/*.avif public/*.png public/*.jpg public/*.jpeg public/*.webp; do
  if [ -f "$f" ]; then cp -f "$f" "$PUBLIC_DIR/"; echo "copié: $f"; fi
done
shopt -u nullglob
if [ -d public/fonts ]; then cp -Rf public/fonts "$PUBLIC_DIR/"; echo "copié: public/fonts"; fi
