import sys


class Counter:
    def __init__(self, value):
        self.value = value

    def bump(self, step):
        self.value = self.value + step
        return self.value


n = int(sys.argv[1]) if len(sys.argv) > 1 else 2000

total = 0
i = 0
while i < n:
    c = Counter(i % 17)
    j = 0
    while j < 10:
        total = (total + c.bump(j + 1)) % 1000000007
        j = j + 1
    total = (total + c.value) % 1000000007
    i = i + 1

print(total)
