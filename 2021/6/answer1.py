import copy 

class Fish:
    def __init__(self, t):
        self.timer = t
        self.done = True

nbs = [int(n) for n in input().split(',')]
print(nbs)

fishes = []
for n in nbs:
    fishes.append(Fish(n))

for d in range(80):
    for f in fishes:
        f.done = False
    for f in fishes:
        if not f.done:
            f.timer = f.timer - 1
            if f.timer < 0:
                fishes.append(Fish(8))
                f.timer = 6

if len(fishes) < 20:
    print([f.timer for f in fishes])
print(len(fishes))
