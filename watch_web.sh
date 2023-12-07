#!/bin/bash
cargo-watch -i "assets" -x "run --target wasm32-unknown-unknown --release"
