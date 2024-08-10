import { defineConfig } from "astro/config";
import tailwind from "@astrojs/tailwind";

import node from "@astrojs/node";

// https://astro.build/config
export default defineConfig({
  devToolbar: {
    enabled: false
  },
  output: "server",
  server: {
    port: 8000
  },
  integrations: [tailwind()],
  adapter: node({
    mode: "middleware"
  }),
  experimental: {
    rewriting: true
  }
});
