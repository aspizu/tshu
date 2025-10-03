#!/usr/bin/env bash
set -e
TARGETS=(
  x86_64-unknown-linux-gnu
  x86_64-pc-windows-msvc
  aarch64-unknown-linux-gnu
  armv7-unknown-linux-gnueabihf
)
uv sync
source .venv/bin/activate
uv pip install maturin
for TARGET in "${TARGETS[@]}"; do
    maturin build \
        --out dist \
        --zig \
        --release \
        --target "$TARGET" \
        --interpreter "python3.14"
done
