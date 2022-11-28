import sys

valid = 0
for line in sys.stdin:
    [p1, p2, p3] = line.split(' ')
    mini, maxi = [int(a) for a in p1.split('-')]
    key = p2[:-1]
    valid += (p3[mini - 1] == key) ^  (p3[maxi - 1] == key)
print(valid)