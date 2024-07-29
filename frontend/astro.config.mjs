import { defineConfig } from 'astro/config';

import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  devToolbar: {
    enabled: true
  },
  output: "server",
  server: {
    host: "127.0.1",
    port: 8001
  },
  integrations: [tailwind()]
});
