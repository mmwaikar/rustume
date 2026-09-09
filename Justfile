# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

cargo-build:
    cargo +nightly build --lib --target wasm32-unknown-unknown --release

gen-wasm:
    wasm-bindgen --target web --out-dir web/wasm target/wasm32-unknown-unknown/release/rustume.wasm

web-build:
    cd web; npm install; npm run build

run-web: cargo-build gen-wasm web-build
    cd web; npm run dev