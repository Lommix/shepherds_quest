#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --no-typescript --target web --out-dir ./html --out-name "shepherds_quest" ./target/wasm32-unknown-unknown/release/bevy_jam_2023.wasm
