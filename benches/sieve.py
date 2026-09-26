import sys

n = int(sys.argv[1]) if len(sys.argv) > 1 else 100

is_prime = []
i = 0
while i <= n:
    is_prime.append(True)
    i = i + 1
is_prime[0] = False
is_prime[1] = False

p = 2
while p * p <= n:
    if is_prime[p]:
        multiple = p * p
        while multiple <= n:
            is_prime[multiple] = False
            multiple = multiple + p
    p = p + 1

count = 0
total = 0
i = 0
while i <= n:
    if is_prime[i]:
        count = count + 1
        total = (total + i) % 1000000007
    i = i + 1

print((count + total) % 1000000007)
