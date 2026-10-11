import sys


class Random:
    def __init__(self):
        self.seed = 74755

    def next(self):
        self.seed = ((self.seed * 1309) + 13849) & 65535
        return self.seed


class Storage:
    def __init__(self):
        self.count = 0

    def benchmark(self):
        random = Random()
        self.count = 0
        self.build_tree_depth(7, random)
        return self.count

    def build_tree_depth(self, depth, random):
        self.count += 1
        if depth == 1:
            return [None] * (random.next() % 10 + 1)
        arr = [None] * 4
        for i in range(4):
            arr[i] = self.build_tree_depth(depth - 1, random)
        return arr


iterations = int(sys.argv[1]) if len(sys.argv) > 1 else 2

storage = Storage()
checksum = 0
for _ in range(iterations):
    result = storage.benchmark()
    if result != 5461:
        raise Exception("Storage verification failed")
    checksum += result
print(checksum)
