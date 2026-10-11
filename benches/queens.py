import sys


class Queens:
    def __init__(self):
        self.free_rows = None
        self.free_maxs = None
        self.free_mins = None
        self.queen_rows = None

    def queens(self):
        self.free_rows = [True] * 8
        self.free_maxs = [True] * 16
        self.free_mins = [True] * 16
        self.queen_rows = [-1] * 8
        return self.place_queen(0)

    def place_queen(self, c):
        for r in range(8):
            if self.get_row_column(r, c):
                self.queen_rows[r] = c
                self.set_row_column(r, c, False)

                if c == 7:
                    return True
                if self.place_queen(c + 1):
                    return True

                self.set_row_column(r, c, True)
        return False

    def get_row_column(self, r, c):
        return self.free_rows[r] and self.free_maxs[c + r] and self.free_mins[c - r + 7]

    def set_row_column(self, r, c, v):
        self.free_rows[r] = v
        self.free_maxs[c + r] = v
        self.free_mins[c - r + 7] = v


def benchmark():
    queens = Queens()
    result = True
    for _ in range(10):
        result = result and queens.queens()
    return result


iterations = int(sys.argv[1]) if len(sys.argv) > 1 else 2

checksum = 0
for _ in range(iterations):
    result = benchmark()
    if not result:
        raise Exception("Queens verification failed")
    checksum += 1
print(checksum)
