import sys


class TreeNode:
    def __init__(self, left, right):
        self.left = left
        self.right = right


def make_tree(depth):
    if depth <= 0:
        return TreeNode(None, None)
    return TreeNode(make_tree(depth - 1), make_tree(depth - 1))


def check_tree(node):
    if node.left is None:
        return 1
    return check_tree(node.left) + check_tree(node.right) + 1


def pow2(k):
    result = 1
    i = 0
    while i < k:
        result = result * 2
        i = i + 1
    return result


n = int(sys.argv[1]) if len(sys.argv) > 1 else 8

min_depth = 4
max_depth = n if n > min_depth + 2 else min_depth + 2

stretch_depth = max_depth + 1
stretch_tree = make_tree(stretch_depth)
checksum = check_tree(stretch_tree) % 1000000007

long_lived_tree = make_tree(max_depth)

depth = min_depth
while depth <= max_depth:
    iterations = pow2(max_depth - depth + min_depth)
    check = 0
    i = 0
    while i < iterations:
        check = check + check_tree(make_tree(depth))
        i = i + 1
    checksum = (checksum + check) % 1000000007
    depth = depth + 2

checksum = (checksum + check_tree(long_lived_tree)) % 1000000007

print(checksum)
