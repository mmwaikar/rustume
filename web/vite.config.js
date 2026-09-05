import { defineConfig } from 'vite';
import { cpSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';

export default defineConfig({
  base: process.env.RUSTUME_BASE_PATH || '/',
  publicDir: '../assets',
  plugins: [{
    name: 'copy-wasm-bindgen-output',
    closeBundle() {
      const wasmSource = resolve('wasm');
      const wasmTarget = resolve('dist/wasm');
      if (existsSync(wasmSource)) cpSync(wasmSource, wasmTarget, { recursive: true });
    }
  }]
});
