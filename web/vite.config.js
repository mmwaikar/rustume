import { defineConfig } from 'vite';
import { cpSync, existsSync, mkdirSync, rmSync } from 'node:fs';
import { resolve } from 'node:path';

const sourceAssets = resolve('../assets');
const publicAssets = resolve('public/assets');
const wasmSource = resolve('wasm');
const wasmTarget = resolve('dist/wasm');

// The wasm asset loader fetches icons from `{origin}/assets/icons/...`,
// so mirror the repo assets under `public/assets` (served at `/assets`).
mkdirSync(publicAssets, { recursive: true });
rmSync(publicAssets, { recursive: true, force: true });
cpSync(sourceAssets, publicAssets, { recursive: true });

export default defineConfig({
  base: process.env.RUSTUME_BASE_PATH || '/',
  publicDir: 'public',
  plugins: [{
    name: 'copy-wasm-bindgen-output',
    closeBundle() {
      if (existsSync(wasmSource)) cpSync(wasmSource, wasmTarget, { recursive: true });
    }
  }]
});