import sys

n = int(sys.argv[1]) if len(sys.argv) > 1 else 100

text = ""
i = 0
while i < n:
    text = text + "ab" + str(i) + ","
    i = i + 1

parts = text.split(",")

checksum = 0
i = 0
while i < len(parts):
    checksum = (checksum + len(parts[i])) % 1000000007
    i = i + 1

replaced = text.replace("ab", "xyz")
checksum = (checksum + len(replaced)) % 1000000007

sub = text[0:10]
checksum = (checksum + len(sub)) % 1000000007

print(checksum)
