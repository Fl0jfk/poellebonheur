function toBase64(value: string): string {
  if (typeof window === "undefined") {
    return Buffer.from(value).toString("base64");
  }
  return window.btoa(value);
}

export function getBlurDataURL(color = "#f3e6d6"): string {
  const svg = `
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 9">
      <defs>
        <linearGradient id="g" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="${color}" />
          <stop offset="100%" stop-color="#ffffff" />
        </linearGradient>
      </defs>
      <rect width="16" height="9" fill="url(#g)" />
    </svg>
  `;
  return `data:image/svg+xml;base64,${toBase64(svg)}`;
}
