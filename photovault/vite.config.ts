import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    port: 1420,
    strictPort: true,
  },
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  strictPort: true,
  // to access the Tauri API use a proxy
  // prevent CORS issues
  proxy: {
    '/api': {
      target: 'http://localhost:1421',
      changeOrigin: true,
      secure: false,
      ws: true,
    },
  },
})