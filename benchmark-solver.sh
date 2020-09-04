#!/bin/bash

set -euo pipefail

ITERATIONS=100
DESTINATION="/tmp/solver-benchmark.txt"

rm -f $DESTINATION
cargo build -q --release

for _ in $(seq 1 $ITERATIONS); do
    cut -d ',' -f 1 levels.txt | while read LEVEL; do
        echo $LEVEL >> $DESTINATION
        bash -c "time ./target/release/golf-peaks --steps" < /tmp/levels/$LEVEL.level.txt 2>&1 >> $DESTINATION | sed -n "s/.*0m\([0-9.]*\)s/\1/p" >> $DESTINATION
    done
done;

python3 -c "
import sys
print('iteration,level,steps,real_time,user_time,system_time')
for i in range(120 * $ITERATIONS):
    iteration = 1 + i // 120
    level = sys.stdin.readline().strip()
    steps = sys.stdin.readline().strip()
    real_time = sys.stdin.readline().strip()
    user_time = sys.stdin.readline().strip()
    system_time = sys.stdin.readline().strip()
    print('{},{},{},{},{},{}'.format(iteration, level, steps, real_time, user_time, system_time))
" < $DESTINATION
