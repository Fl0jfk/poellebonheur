import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "La Poêlée du Bonheur — Traiteur événementielle",
  description: "Traiteur poêlées en Normandie pour vos événements : mariages, anniversaires, foires, marchés... Intervention dans toute la Normandie. Devis gratuit.",
};

export default function RootLayout({children,}: Readonly<{children: React.ReactNode}>) {
  return (
    <html lang="fr">
      <body>{children}</body>
    </html>
  );
}
