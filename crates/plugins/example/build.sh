#!/bin/sh
# Builds the Desaturate example plug-in and refreshes the test fixture built from it.
# Needs the wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`.
set -eu
cd "$(dirname "$0")"
# The plug-in runtime takes WebAssembly 1.0 (MVP): newer compilers turn on SIMD and more by default.
RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown
out=target/wasm32-unknown-unknown/release/pluma_plugin_desaturate.wasm
cp "$out" ../tests/fixtures/desaturate.wasm
echo "built $out ($(wc -c < "$out") bytes) -> crates/plugins/tests/fixtures/desaturate.wasm"
