import sys


class Permute:
    def __init__(self):
        self.count = 0
        self.v = None

    def benchmark(self):
        self.count = 0
        self.v = [0] * 6
        self.permute(6)
        return self.count

    def permute(self, n):
        self.count += 1
        if n != 0:
            n1 = n - 1
            self.permute(n1)
            for i in range(n1, -1, -1):
                self.swap(n1, i)
                self.permute(n1)
                self.swap(n1, i)

    def swap(self, i, j):
        tmp = self.v[i]
        self.v[i] = self.v[j]
        self.v[j] = tmp


iterations = int(sys.argv[1]) if len(sys.argv) > 1 else 2

permute = Permute()
checksum = 0
for _ in range(iterations):
    result = permute.benchmark()
    if result != 8660:
        raise Exception("Permute verification failed")
    checksum += result
print(checksum)
