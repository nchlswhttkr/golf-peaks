#!/bin/bash

set -euo pipefail

ls -1 levels/* | while read LEVEL; do
    cargo run -q -- --applescript < $LEVEL | osascript -i
    sleep 3
done;