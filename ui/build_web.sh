#!/bin/bash
set -eu

# ./setup_web.sh # <- call this first!
TARGET_DIR="$(cargo metadata  | jq -r  '.target_directory')"
CRATE_NAME="$(cargo metadata  | jq -r .resolve.root | cut -d' ' -f1)"
CRATE_NAME_SNAKE_CASE="${CRATE_NAME//-/_}" # for those who name crates with-kebab-case

# This is required to enable the web_sys clipboard API which egui_web uses
# https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Clipboard.html
# https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html
export RUSTFLAGS=--cfg=web_sys_unstable_apis

# Clear output from old stuff:
rm -f docs/"${CRATE_NAME_SNAKE_CASE}"_bg.wasm

echo "Building rust…"
BUILD=release
cargo build --release --lib --target wasm32-unknown-unknown

echo "Generating JS bindings for wasm…"
TARGET_NAME="${CRATE_NAME_SNAKE_CASE}.wasm"
wasm-bindgen "${TARGET_DIR}/wasm32-unknown-unknown/${BUILD}/${TARGET_NAME}" \
  --out-dir docs --no-modules --no-typescript

# to get wasm-opt:  apt/brew/dnf install binaryen
echo "Optimizing wasm…"
wasm-opt docs/"${CRATE_NAME_SNAKE_CASE}"_bg.wasm -O2 --fast-math -o docs/"${CRATE_NAME_SNAKE_CASE}"_bg.wasm # add -g to get debug symbols

echo "Finished: docs/${CRATE_NAME_SNAKE_CASE}.wasm"
