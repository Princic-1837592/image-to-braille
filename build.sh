cd wasm
rm pkg/*
wasm-pack build --target web
rm pkg/.gitignore pkg/package.json pkg/wasm.d.ts pkg/wasm_bg.wasm.d.ts
git add -f pkg/*wasm.js pkg/*wasm_bg.wasm
read -p Press Enter to close...
