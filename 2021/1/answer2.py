import sys

numbs = [int(line) for line in sys.stdin]
s = 0
for i in range(3, len(numbs)):
    s = s + (numbs[i] > numbs[i-3])
print(s)