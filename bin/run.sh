#!/usr/bin/env sh
set -eu

cd "$(dirname "$0")/.."

EXECUTABLE_NAME="$(basename "$(pwd)")"

cargo build

exec "$(pwd)/target/debug/${EXECUTABLE_NAME}" "$@"
