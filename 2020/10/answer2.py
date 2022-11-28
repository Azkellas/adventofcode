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

length = len(nbs)
def rec(p_idx, p_size):
    if p_idx == length:
        return p_size == nbs[-1]

    if nbs[p_idx] > p_size + 3:
        return 0
    
    tot = 0
    tot += rec(p_idx + 1, p_size)
    tot += rec(p_idx + 1, nbs[p_idx])

    return tot


# rec(0, 0)

maxi = max(nbs)

counts = [0 for i in range(maxi+1)]

counts[-1] = 1
for size in range(maxi-1, 0, -1):
    counts[size] = 0
    if size in nbs:
        for i in [1, 2, 3]:
            if size + i in nbs:
                counts[size] += counts[size + i]
counts[0] = sum(counts[1:4])

print(counts[0])


counts = [0 for i in range(length)]

counts[-1] = 1
for size in range(length-2, 0, -1):
    counts[size] = 0
    for i in [1, 2, 3]:
        if size + i < length and nbs[size + i] <= nbs[size] + 3:
            counts[size] += counts[size + i]

for i in range(3):
    if size + i < length and nbs[i] <= 3:
        counts[0] += counts[i]

print(counts[0])

print(nbs)
print(counts)
