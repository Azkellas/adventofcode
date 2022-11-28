import sys

valid = 0
for line in sys.stdin:
    [p1, p2, p3] = line.split(' ')
    mini, maxi = [int(a) for a in p1.split('-')]
    key = p2[:-1]
    valid += mini <= p3.count(key) <= maxi
print(valid)