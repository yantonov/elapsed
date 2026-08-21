#!/bin/sh

cd "$(dirname "$0")"

cd ..

EXECUTABLE_NAME="$(basename $(pwd))"

TARGET="$(pwd)/target/release/${EXECUTABLE_NAME}"

cargo build --release

echo "binary file is here: ${TARGET}"

# reduce binary size
if command -v strip >/dev/null 2>&1; then
    strip "${TARGET}"
else
    echo "strip not found, skipping binary size reduction"
fi
