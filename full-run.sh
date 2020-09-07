#!/bin/bash

set -euo pipefail

function complete_level {
    ASSET=$(grep -m 1 "$1" levels.txt | cut -d ',' -f 2)
    echo "Solving level $1"
    /usr/local/bin/python3.8 parse.py < gp_levels/$ASSET.asset | ./target/release/golf-peaks --applescript | osascript -i
    sleep 2
}

function next_world {
    osascript -e '
tell application "System Events" to key code 36
delay 1.5
'
}

function challenge_levels {
    osascript -e '
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to keystroke "a"
delay 0.05
tell application "System Events" to key code 36
delay 1
'
}

cargo build -q --release
complete_level 01-01
complete_level 01-02
complete_level 01-03
complete_level 01-04
complete_level 01-05
complete_level 01-06
complete_level 01-07
complete_level 01-08
complete_level 01-09
challenge_levels
complete_level 01-10
complete_level 01-11
complete_level 01-12
next_world
complete_level 02-01
complete_level 02-02
complete_level 02-03
complete_level 02-04
complete_level 02-05
complete_level 02-06
complete_level 02-07
complete_level 02-08
complete_level 02-09
challenge_levels
complete_level 02-10
complete_level 02-11
complete_level 02-12
next_world
complete_level 03-01
complete_level 03-02
complete_level 03-03
complete_level 03-04
complete_level 03-05
complete_level 03-06
complete_level 03-07
complete_level 03-08
complete_level 03-09
challenge_levels
complete_level 03-10
complete_level 03-11
complete_level 03-12
next_world
complete_level 04-01
complete_level 04-02
complete_level 04-03
complete_level 04-04
complete_level 04-05
complete_level 04-06
complete_level 04-07
complete_level 04-08
complete_level 04-09
challenge_levels
complete_level 04-10
complete_level 04-11
complete_level 04-12
next_world
complete_level 05-01
complete_level 05-02
complete_level 05-03
complete_level 05-04
complete_level 05-05
complete_level 05-06
complete_level 05-07
complete_level 05-08
complete_level 05-09
challenge_levels
complete_level 05-10
complete_level 05-11
complete_level 05-12
next_world
complete_level 06-01
complete_level 06-02
complete_level 06-03
complete_level 06-04
complete_level 06-05
complete_level 06-06
complete_level 06-07
complete_level 06-08
complete_level 06-09
challenge_levels
complete_level 06-10
complete_level 06-11
complete_level 06-12
next_world
complete_level 07-01
complete_level 07-02
complete_level 07-03
complete_level 07-04
complete_level 07-05
complete_level 07-06
complete_level 07-07
complete_level 07-08
complete_level 07-09
complete_level 07-10
complete_level 07-11
complete_level 07-12
next_world
complete_level 08-01
complete_level 08-02
complete_level 08-03
complete_level 08-04
complete_level 08-05
complete_level 08-06
complete_level 08-07
complete_level 08-08
complete_level 08-09
challenge_levels
complete_level 08-10
complete_level 08-11
complete_level 08-12
next_world
complete_level 09-01
complete_level 09-02
complete_level 09-03
complete_level 09-04
complete_level 09-05
complete_level 09-06
complete_level 09-07
complete_level 09-08
complete_level 09-09
challenge_levels
complete_level 09-10
complete_level 09-11
complete_level 09-12
next_world
complete_level 10-01
complete_level 10-02
complete_level 10-03
complete_level 10-04
complete_level 10-05
complete_level 10-06
complete_level 10-07
complete_level 10-08
complete_level 10-09
complete_level 10-10
complete_level 10-11
complete_level 10-12
next_world
