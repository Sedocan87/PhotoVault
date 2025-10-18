import path from "path"
import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  // Vite optons tailored for Tauri developemnt and only applied in `tauri dev` or `tauri build`
  ...(process.env.TAURI_DEBUG || process.env.TAURI_BUILD
    ? {
        // prevent vite from obscuring rust errors
        clearScreen: false,
        // tauri expects a fixed port, fail if that port is not available
        server: {
          port: 1420,
          strictPort: true,
        },
        // to make use of `TAURI_DEBUG` and other env variables
        // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
        envPrefix: ["VITE_", "TAURI_"],
        build: {
          // Tauri supports es2021
          target: ["es2021", "chrome100", "safari13"],
          // don't minify for debug builds
          minify: false,
          // produce sourcemaps for debug builds
          sourcemap: false,
        },
      }
    : {}),
})