import sys

DEFAULT = 15


def fib(n):
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)


n = int(sys.argv[1]) if len(sys.argv) > 1 else DEFAULT
print(fib(n))
