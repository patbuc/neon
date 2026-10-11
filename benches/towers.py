import sys


class TowersDisk:
    def __init__(self, size):
        self.size = size
        self.next = None


class Towers:
    def __init__(self):
        self.piles = None
        self.moves_done = 0

    def push_disk(self, disk, pile):
        top = self.piles[pile]
        if top is not None and disk.size >= top.size:
            raise Exception("Cannot put a big disk on a smaller one")
        disk.next = top
        self.piles[pile] = disk

    def pop_disk_from(self, pile):
        top = self.piles[pile]
        if top is None:
            raise Exception("Attempting to remove a disk from an empty pile")
        self.piles[pile] = top.next
        top.next = None
        return top

    def move_top_disk(self, from_pile, to_pile):
        self.push_disk(self.pop_disk_from(from_pile), to_pile)
        self.moves_done += 1

    def build_tower_at(self, pile, disks):
        for i in range(disks, -1, -1):
            self.push_disk(TowersDisk(i), pile)

    def move_disks(self, disks, from_pile, to_pile):
        if disks == 1:
            self.move_top_disk(from_pile, to_pile)
        else:
            other_pile = (3 - from_pile) - to_pile
            self.move_disks(disks - 1, from_pile, other_pile)
            self.move_top_disk(from_pile, to_pile)
            self.move_disks(disks - 1, other_pile, to_pile)


def benchmark():
    towers = Towers()
    towers.piles = [None, None, None]
    towers.build_tower_at(0, 13)
    towers.moves_done = 0
    towers.move_disks(13, 0, 1)
    return towers.moves_done


iterations = int(sys.argv[1]) if len(sys.argv) > 1 else 2

checksum = 0
for _ in range(iterations):
    result = benchmark()
    if result != 8191:
        raise Exception(f"Towers verification failed: {result}")
    checksum += result
print(checksum)
