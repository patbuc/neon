#!/usr/bin/env python3
# Nested loops benchmark
# Tests loop performance and variable assignment

sum = 0
for i in range(1000):
    for j in range(1000):
        sum += 1
print(sum)
