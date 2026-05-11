export type MenuCategoryNorm = "starter" | "main_dish" | "dessert";

function foldAccents(s: string) {
  return s.normalize("NFD").replace(/\p{M}/gu, "");
}

/** Lowercase, espaces et tirets → underscores pour comparer des libellés variés */
function sepKey(s: string) {
  return foldAccents(s)
    .trim()
    .toLowerCase()
    .replace(/[\s-]+/g, "_")
    .replace(/_+/g, "_");
}

/**
 * Harmonise les catégories (admin, JSON manuel, anciennes valeurs) vers starter | main_dish | dessert.
 */
export function normalizeMenuCategory(raw: string): MenuCategoryNorm {
  if (raw == null || typeof raw !== "string") return "starter";
  const n = sepKey(raw);
  if (!n) return "starter";
  if (n === "maindish" || n === "main_dish" || n === "main") return "main_dish";
  if (n === "plat_principal" || n === "plats_principaux" || n === "platprincipal" || n === "platsprincipaux") {
    return "main_dish";
  }
  if (n === "dessert" || n === "desserts") return "dessert";
  if (n === "entree" || n === "entrees" || n === "starter" || n === "appetizer") return "starter";
  return "starter";
}
