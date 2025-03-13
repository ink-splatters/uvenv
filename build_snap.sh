#!/bin/bash
set -e

# special build
cargo build --release --features snap

cp ./venv/bin/uv target/release/uv

snapcraft

# todo: publish
