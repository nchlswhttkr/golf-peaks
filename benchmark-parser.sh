#!/bin/bash

set -euo pipefail

ITERATIONS=100
DESTINATION="/tmp/parser-benchmark.txt"

rm -f $DESTINATION

for _ in $(seq 1 $ITERATIONS); do
    cut -d ',' -f 2 levels.txt | while read ASSET; do
        grep -m 1 "$ASSET" levels.txt | cut -d ',' -f 1 >> $DESTINATION
        bash -c "time /usr/local/bin/python3.8 parse.py" < gp_levels/$ASSET.asset 2>&1 > /dev/null | sed -n "s/.*0m\([0-9.]*\)s/\1/p" >> $DESTINATION
    done
done;

python3 -c "
import sys
print('iteration,level,steps,real_time,user_time,system_time')
for i in range(120 * $ITERATIONS):
    iteration = 1 + i // 120
    level = sys.stdin.readline().strip()
    real_time = sys.stdin.readline().strip()
    user_time = sys.stdin.readline().strip()
    system_time = sys.stdin.readline().strip()
    print('{},{},0,{},{},{}'.format(iteration, level, real_time, user_time, system_time))
" < $DESTINATION
