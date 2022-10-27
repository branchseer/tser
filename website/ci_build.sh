#!/usr/bin/env bash
set -e

npm run build-wasm
npm run check
npm run build
