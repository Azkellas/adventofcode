from sys import stdin

tot = 0

for line in stdin:
    arr = [int(i) for i in line.split('\t')]
    ma = 0
    mi = 1e12
    for i in arr:
        ma = max(ma, i)
        mi = min(mi, i)
    tot += ma - mi
print(tot)