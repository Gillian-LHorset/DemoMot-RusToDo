import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueDevTools from "vite-plugin-vue-devtools";

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue(), vueDevTools()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  server: {
    proxy: {
      // Dès que Axios fera une requête commençant par "/api", Vite prendra le relais
      "/api": {
        target: "http://localhost:5000", // L'adresse de ton backend Rust
        changeOrigin: true,
        //rewrite: (path) => path.replace(/^\/api/, ""), // Supprime "/api" avant d'envoyer au Rust
      },
    },
  },
});
