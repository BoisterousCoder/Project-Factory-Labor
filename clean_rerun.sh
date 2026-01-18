#!/bin/bash
# My computer is really good at corrupting my rust libraries so this cleans and runs cargo run
# Remember to stop the Rust Lint service using ctrl+shift+p first
rm -rf target/
rm -rf Cargo.lock
export RUST_BACKTRACE=1
cargo run