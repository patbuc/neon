import sys

n = int(sys.argv[1]) if len(sys.argv) > 1 else 50

arr = []
i = 0
while i < n:
    arr.append(i * 2)
    i = i + 1

m = {}
i = 0
while i < n:
    m["key" + str(i)] = i
    i = i + 1

s = set()
i = 0
while i < n:
    s.add(i * 3)
    i = i + 1

checksum = 0
i = 0
while i < n:
    checksum = (checksum + arr[i]) % 1000000007
    key = "key" + str(i)
    if key in m:
        checksum = (checksum + m[key]) % 1000000007
    if i in s:
        checksum = (checksum + 1) % 1000000007
    i = i + 1

print(checksum)
