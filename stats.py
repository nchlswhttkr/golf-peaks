import sys

process_time = 0
lines_read = 0
total_steps = 0

sys.stdin.readline()
for line in sys.stdin.readlines():
    lines_read += 1
    total_steps += int(line.split(',')[2])
    process_time += sum([float(i) for i in line.strip().split(',')[4:5]])

iterations = lines_read // 120

print('Ran {} iterations'.format(iterations))
print('Took {:.3f}s per iteration'.format(process_time / iterations))
print('Solving all levels took {} steps'.format(total_steps // iterations))
