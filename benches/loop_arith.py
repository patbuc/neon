import sys

n = int(sys.argv[1]) if len(sys.argv) > 1 else 2000

total = 0
i = 0
while i < n:
    total = (total + i * 3 - i % 7) % 1000000007
    i = i + 1

for j in range(n):
    total = (total * 31 + j) % 1000000007

print(total)
