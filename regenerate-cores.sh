#! /bin/bash -ea

set -o pipefail

SCRIPT_DIR=$(dirname $0)

cd "${SCRIPT_DIR}/core_generator"

cargo run "../src"

