import { defineConfig } from 'vite';

export default defineConfig({
  base: process.env.RUSTUME_BASE_PATH || '/',
  publicDir: '../assets'
});
