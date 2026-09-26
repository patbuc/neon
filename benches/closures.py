import sys

n = int(sys.argv[1]) if len(sys.argv) > 1 else 500


def make_counter(start):
    value = start

    def inc(step):
        nonlocal value
        value = value + step
        return value

    return inc


total = 0
i = 0
while i < n:
    counter = make_counter(i % 13)
    j = 0
    while j < 20:
        total = (total + counter(j + 1)) % 1000000007
        j = j + 1
    i = i + 1

print(total)
