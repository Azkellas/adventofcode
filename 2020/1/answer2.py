import sys

numbs = [int(line) for line in sys.stdin]
for a in numbs:
    for b in numbs:
        for c in numbs:
            if a + b + c == 2020:
                print(a*b*c)