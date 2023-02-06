#!/usr/bin/env bash
set -euxo pipefail

cargo bin cargo-watch --exec run
