import sys

numbs = [int(line) for line in sys.stdin]
for a in numbs:
    for b in numbs:
        if a + b == 2020:
            print(a*b)