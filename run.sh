#!/bin/bash

# $1 - Level ID in Golf Peaks, for example 01-02

set -euo pipefail

LEVEL=$(grep -m 1 "$1" levels.txt | cut -d ',' -f 2)
echo "Solving level $1"
# cat gp_levels/$LEVEL.asset | python3 parse.py | cargo run -q
cat gp_levels/$LEVEL.asset | python3 parse.py | cargo run -q -- --applescript | osascript -i
