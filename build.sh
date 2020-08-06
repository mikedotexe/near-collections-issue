#!/bin/bash
cargo +stable build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/near_collections_issues.wasm res/