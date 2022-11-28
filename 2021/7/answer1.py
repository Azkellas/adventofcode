nbs = [int(n) for n in input().split(',')]
nbs.sort()

l = len(nbs)
if l % 2 == 0:
    m = round((nbs[l // 2 - 1] + nbs[l // 2]) / 2)
else:
    m = nbs[len(nbs)//2 + 1]

# m = nbs[len(nbs)//2]

print(l)
print(m)
print(sum([abs(n-m) for n in nbs]))