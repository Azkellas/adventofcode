import sys
import math

mol = input()
input()

cors = {}
letters = set()
for line in sys.stdin:
    print(line.strip())
    a, o, b = line.split()
    cors[a] = b
    letters.add(a[0])
    letters.add(a[1])
    letters.add(b)

print(cors)

pairs = {}
for i in range(len(mol) - 1):
    s = mol[i:i+2]
    if s not in pairs: pairs[s] = 0
    pairs[s] = pairs[s] + 1

print(pairs)

for step in range(40):
    print(step, pairs)
    print("len ", sum(pairs.values()) + 1)
    npairs = {}
    for pair in pairs:
        o = cors[pair]
        s1 = pair[0] + o
        s2 = o + pair[1]
        if s1 not in npairs: npairs[s1] = 0
        if s2 not in npairs: npairs[s2] = 0
        npairs[s1] = npairs[s1] + pairs[pair]
        npairs[s2] = npairs[s2] + pairs[pair]
    pairs = npairs
print(step, pairs)
print("len ", sum(pairs.values()) + 1)

maxi = 0
mini = 1_000_000_000_000

for letter in letters:
    c = 0
    for p in pairs:
        c = c + p.count(letter) * pairs[p]
    c = math.ceil(c / 2)
    if c > maxi: maxi = c
    if c < mini: mini = c
    print(f"{letter} = {c}")

print(f"{maxi}-{mini} = {maxi-mini}")