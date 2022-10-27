#!/usr/bin/env bash
set -e

cargo install --version=0.10.3 wasm-pack
npm run check
npm run build-wasm
npm run build
