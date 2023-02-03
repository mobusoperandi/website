#!/usr/bin/env bash
set -euxo pipefail
toc=$(cargo bin md-toc --header "# Table of contents" README.md)
MARKER='<!-- TOC -->'
cargo bin sd "$MARKER[\S\s]*$MARKER" $"$MARKER$toc\n$MARKER" README.md
