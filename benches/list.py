import sys


class Element:
    def __init__(self, value):
        self.value = value
        self.next = None

    def length(self):
        if self.next is None:
            return 1
        return 1 + self.next.length()


def make_list(length):
    if length == 0:
        return None
    e = Element(length)
    e.next = make_list(length - 1)
    return e


def is_shorter_than(x, y):
    x_tail = x
    y_tail = y

    while y_tail is not None:
        if x_tail is None:
            return True
        x_tail = x_tail.next
        y_tail = y_tail.next

    return False


def tail(x, y, z):
    if is_shorter_than(y, x):
        return tail(tail(x.next, y, z), tail(y.next, z, x), tail(z.next, x, y))
    return z


def benchmark():
    result = tail(make_list(15), make_list(10), make_list(6))
    return result.length()


iterations = int(sys.argv[1]) if len(sys.argv) > 1 else 2

checksum = 0
for _ in range(iterations):
    result = benchmark()
    if result != 10:
        raise Exception("List verification failed")
    checksum += result
print(checksum)
