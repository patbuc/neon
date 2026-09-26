import sys

n = int(sys.argv[1]) if len(sys.argv) > 1 else 100

checksum = 0
i = 0
while i < n:
    s = "ab" + str(i) + ",cd,ef"
    parts = s.split(",")
    j = 0
    while j < len(parts):
        checksum = (checksum + len(parts[j])) % 1000000007
        j = j + 1
    replaced = s.replace("ab", "xyz")
    checksum = (checksum + len(replaced)) % 1000000007
    sub = s[0:4]
    checksum = (checksum + len(sub)) % 1000000007
    i = i + 1

print(checksum)
