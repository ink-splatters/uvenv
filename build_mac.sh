#!/bin/bash
set -e

rm -rf target/wheels/

. venv/bin/activate

# without zig because that breaks stuff on mac
maturin build --release --strip --target aarch64-apple-darwin #--zig
maturin build --release --strip --target x86_64-apple-darwin #--zig

maturin upload --skip-existing -u __token__ target/wheels/*
