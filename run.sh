#!/bin/bash

set -euo pipefail

ls -1 gp_levels/roll_[1-9].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

# complete challenge levels
osascript -e '
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to key code 36
delay 2
'

ls -1 gp_levels/roll_1[0-2].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

# move on to next world
osascript -e '
tell application "System Events" to key code 36
delay 2
'

ls -1 gp_levels/air_[1-9].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

# complete challenge levels
osascript -e '
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to key code 36
delay 2
'

ls -1 gp_levels/air_1[0-2].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

# move on to next world
osascript -e '
tell application "System Events" to key code 36
delay 2
'

ls -1 gp_levels/sand_[1-9].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

# complete challenge levels
osascript -e '
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to key code 36
delay 2
'

ls -1 gp_levels/sand_1[0-2].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

# move on to next world
osascript -e '
tell application "System Events" to key code 36
delay 2
'

ls -1 gp_levels/water_[1-9].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

# complete challenge levels
osascript -e '
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to key code 36
delay 2
'

ls -1 gp_levels/water_1[0-2].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

# move on to next world
osascript -e '
tell application "System Events" to key code 36
delay 2
'

ls -1 gp_levels/spring_[1-9].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

# complete challenge levels
osascript -e '
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to key code 36
delay 2
'

ls -1 gp_levels/spring_1[0-2].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

# move on to next world
osascript -e '
tell application "System Events" to key code 36
delay 2
'

ls -1 gp_levels/portal_[1-9].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

# complete challenge levels
osascript -e '
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to key code 36
delay 2
'

ls -1 gp_levels/portal_1[0-2].asset | while read LEVEL; do
    cat $LEVEL | python3 parse.py | cargo run -q -- --applescript | osascript -i
    sleep 3
done;

curl parrot.live
