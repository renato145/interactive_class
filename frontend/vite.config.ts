import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {
  if (command === "serve") {
    // dev specific config
    return {
      plugins: [svelte()],
      server: {
        proxy: {
          "/cups": "http://localhost:8000",
        },
      },
    };
  } else {
    // build specific config
    return {
      plugins: [svelte()],
    };
  }
});
