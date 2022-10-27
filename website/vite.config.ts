import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import wasm from "vite-plugin-wasm";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    wasm(),
    svelte(),
  ],
  optimizeDeps: {
    exclude: ["codemirror", "@codemirror/language", "@codemirror/legacy-modes/mode/swift", "@codemirror/state"]
  },
  build: {
    target: "es2022"
  }
})
