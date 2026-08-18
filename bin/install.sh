#!/usr/bin/env sh
set -eu

cd "$(dirname "$0")/.."

./bin/release.sh

EXECUTABLE_NAME="$(basename $(pwd))"

TARGET_DIR="${HOME}/.local/bin"

mkdir -p "${TARGET_DIR}"

TARGET="${TARGET_DIR}/${EXECUTABLE_NAME}"
if [ -f "${TARGET}" ] || [ -L "${TARGET}" ]; then
    echo "Remove old file ${TARGET}"
    rm "${TARGET}"
fi

cp "$(pwd)/target/release/${EXECUTABLE_NAME}" "${TARGET}"

echo "Installed to ${TARGET}"
