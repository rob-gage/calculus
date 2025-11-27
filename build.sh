#!/bin/bash

set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

trunk build --release
echo "calculus.dogwood.cloud" > "$SCRIPT_DIR/docs/CNAME"