const path = require("path");

/** @type {import('next').NextConfig} */
const nextConfig = {
  images: {
    formats: ["image/avif", "image/webp"],
    minimumCacheTTL: 2678400,
  },
  outputFileTracingRoot: path.join(process.cwd()),
};

module.exports = nextConfig;
