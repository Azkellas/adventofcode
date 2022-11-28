nbs = [int(n) for n in input().split(',')]
nbs.sort()

l = len(nbs)

maxi = max(nbs) + 1
min_score = 1000000000
for m in range(maxi):
    dst = [abs(m - n) for n in nbs]
    score = sum([d*(d+1)/2 for d in dst])
    if score < min_score:
        min_score = score

print(l)
print(min_score)
