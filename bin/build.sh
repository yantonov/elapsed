#!/bin/sh
set -eu

cd "$(dirname "$0")"

cd ..

EXECUTABLE_NAME="$(basename $(pwd))"

TARGET="$(pwd)/target/debug/${EXECUTABLE_NAME}"

cargo fmt --all -- --check

cargo clippy --all-targets --all-features -- -D warnings

cargo build

echo "binary file is here: ${TARGET}"
