#!/bin/bash

set -e

# Prepare level files
mkdir -p /tmp/levels
rm -f /tmp/levels/*
cut -d "," -f 1 levels.txt | while read LEVEL; do    ASSET=$(grep $LEVEL levels.txt | cut -d "," -f 2)
    python3 parse.py < gp_levels/$ASSET.asset > /tmp/levels/$LEVEL.level.txt
done

# Initial run
git checkout -q 56f29a68cc28bc137a30d562915c21f85f6d8041
./benchmark-solver.sh > benchmarks/benchmark-solver-01.csv
echo "Completed 01"
python3 stats.py < benchmarks/benchmark-solver-01.csv
sleep 30

# Discard paths with loops
git checkout -q 33f7dfac525feb3ed3a6808739074ac0d9a57955
./benchmark-solver.sh > benchmarks/benchmark-solver-02.csv
echo "Completed 02"
python3 stats.py < benchmarks/benchmark-solver-02.csv
sleep 30

# Make position history mutable
git checkout -q e7b50cdba77cf3abe37d1df9b6e8dcdf64d0587b
./benchmark-solver.sh > benchmarks/benchmark-solver-03.csv
echo "Completed 03"
python3 stats.py < benchmarks/benchmark-solver-03.csv
sleep 30

# Add memoization for moves
git checkout -q c286219e32b21eff20a9f24e38f56a7b085be7ff
./benchmark-solver.sh > benchmarks/benchmark-solver-04.csv
echo "Completed 04"
python3 stats.py < benchmarks/benchmark-solver-04.csv
sleep 30

# Try all paths and return shortest
git checkout -q b147d7eddc0440b9009c5fdc0e161a8d5a5ee6e1
./benchmark-solver.sh > benchmarks/benchmark-solver-05.csv
echo "Completed 05"
python3 stats.py < benchmarks/benchmark-solver-05.csv
sleep 30

# Discard paths that can't beat step count
git checkout -q 05ed4b239f5b8c744ff8e69ff8853e7c6bed2dec
./benchmark-solver.sh > benchmarks/benchmark-solver-06.csv
echo "Completed 06"
python3 stats.py < benchmarks/benchmark-solver-06.csv
