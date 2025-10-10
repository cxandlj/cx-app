import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from 'path';
import monacoEditorEsmPlugin from 'vite-plugin-monaco-editor-esm'

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/1
export default defineConfig(async () => ({
  plugins: [vue(), monacoEditorEsmPlugin()],

  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
