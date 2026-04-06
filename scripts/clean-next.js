/**
 * Supprime le dossier .next pour éviter l’erreur récurrente
 * "Cannot find module './331.js'" (cache webpack désynchronisé).
 * Appelé automatiquement avant `npm run dev`.
 */
const fs = require("node:fs");
const path = require("node:path");

const nextDir = path.join(__dirname, "..", ".next");
try {
  fs.rmSync(nextDir, { recursive: true, force: true });
} catch {
  /* ignore */
}
