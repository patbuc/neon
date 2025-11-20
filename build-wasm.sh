#!/bin/bash

# Build script for WebAssembly target
# Requires wasm-pack: cargo install wasm-pack

set -e

echo "Building Neon for WebAssembly..."

# Build with wasm-pack
wasm-pack build --target web --out-dir wasm-pkg

echo "Build complete! Output in ./wasm-pkg/"
echo ""
echo "Files generated:"
echo "  - wasm-pkg/neon_bg.wasm  (WebAssembly binary)"
echo "  - wasm-pkg/neon.js       (JavaScript bindings)"
echo "  - wasm-pkg/neon.d.ts     (TypeScript definitions)"
echo ""
echo "To use in a web page, see demo/index.html"
