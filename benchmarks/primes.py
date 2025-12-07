#!/usr/bin/env python3
# Prime number counting benchmark
# Tests arithmetic operations, conditionals, and nested loops

def is_prime(n):
    if n <= 1:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    i = 3
    while i * i <= n:
        if n % i == 0:
            return False
        i += 2
    return True

count = 0
n = 2
while n < 10000:
    if is_prime(n):
        count += 1
    n += 1
print(count)
