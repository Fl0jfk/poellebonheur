const path = require("path");

/** @type {import('next').NextConfig} */
const nextConfig = {
  images: { unoptimized: true },
  outputFileTracingRoot: path.join(process.cwd()),
};

module.exports = nextConfig;
