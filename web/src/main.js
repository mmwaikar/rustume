const status = document.querySelector('#boot-status');
try {
  const wasm = await import(/* @vite-ignore */ `${import.meta.env.BASE_URL}wasm/rustume.js`);
  await wasm.default();
  status.remove();
} catch (error) {
  status.textContent = 'Rustume could not start. Use a current browser with WebAssembly and WebGPU support.';
  console.error(error);
}
