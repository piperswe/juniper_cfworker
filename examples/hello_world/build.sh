#!/usr/bin/env bash

set -euxo pipefail

export CARGO_INSTALL_ROOT="$(pwd)/target/bin"
export PATH="${CARGO_INSTALL_ROOT}:${PATH}"
cargo install --root ./target worker-build wasm-pack
worker-build --release
wasm-opt -Oz -o ./build/worker/index_bg{_opt,}.wasm
mv ./build/worker/index_bg{_opt,}.wasm