import sys

numbs = [int(line) for line in sys.stdin]
s = 0
for i in range(1, len(numbs)):
    s = s + (numbs[i] > numbs[i-1])
print(s)