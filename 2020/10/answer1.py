import sys

nbs = []

for line in sys.stdin:
    nbs.append(int(line))



nbs.sort()

d1 = nbs[0] == 1
d3 = 1 + (nbs[0] == 3)
for i in range(len(nbs) - 1):
    d1 += (nbs[i+1] - nbs[i] == 1)
    d3 += (nbs[i+1] - nbs[i] == 3)

print(f'{d1} * {d3} = {d1 * d3}')