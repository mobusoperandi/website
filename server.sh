#!/usr/bin/env bash
set -euxo pipefail

mkdir -p $OUTPUT_DIR
cargo bin live-server --host localhost $OUTPUT_DIR
