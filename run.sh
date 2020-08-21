#!/bin/bash

set -euo pipefail

osascript -e '
activate application "Golf Peaks"
'

ls -1 levels/01-0* | while read LEVEL; do
    cargo run -q -- --applescript < $LEVEL | osascript -i
    sleep 3
done;

# complete challenge levels
osascript -e '
tell application "System Events" to keystroke "a"
delay 0.1
tell application "System Events" to keystroke "a"
delay 0.1
tell application "System Events" to keystroke "a"
delay 0.1
tell application "System Events" to key code 36
delay 3
'

ls -1 levels/01-1* | while read LEVEL; do
    cargo run -q -- --applescript < $LEVEL | osascript -i
    sleep 3
done;
