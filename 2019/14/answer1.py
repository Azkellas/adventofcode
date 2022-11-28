from sys import stdin
import re
import math

class Comp:
    def __init__(self, name, val):
        self.name = name
        self.count = val

    def __str__(self):
        return f"{self.count} {self.name}"

class Eq:
    def __init__(self, name="", val=0):
        self.res = Comp(name, val)
        self.reacts = []

    def __str__(self):
        s = ""
        for c in self.reacts:
            s += str(c) + " + "
        s = s[:-3]
        s += " => " + str(self.res)
        return s

def parseEq(line):
    res = line.split(' ')
    idx = 0
    reactive = True
    eq = Eq()
    while idx < len(res):
        c = res[idx]
        if c == '=>':
            idx += 1
            reactive = False
            continue

        c = int(c)
        n = res[idx+1]
        if n[-1] == ',':
            n = n[:-1]

        idx += 2
        if reactive:
            eq.reacts.append(Comp(n, c))
        else:
            eq.res = Comp(n, c)
    print(eq)
    return eq

eqs = {}
curr = {}
depths = {}
depths['ORE'] = 0
curr['FUEL'] = 1

for line in stdin:
    line = line.strip()
    print(line)
    eq = parseEq(line)
    eqs[eq.res.name] = eq

# compute depths
working = True
while working:
    working = False
    for eq in eqs:
        eq = eqs[eq]
        if eq.res.name in depths:
            continue

        valid = True
        maxD = -1
        for c in eq.reacts:
            if not c.name in depths:
                valid = False
                break
            else:
                maxD = max(maxD, depths[c.name])
        if valid:
            working = True
            depths[eq.res.name] = maxD + 1

print(depths)

    # for name in curr:
    #     count = curr[name]
    #     if name != "FUEL":
    #         working = True

while True:
    minDepth = -1
    for name in curr:
        if curr[name] > 0 and name != "ORE" and depths[name] > minDepth:
            react = name
            minDepth = depths[name]

    if minDepth == -1:
        break

    print("reaction for ", react)
    eq = eqs[react]
    count = curr[react]
    mult = math.ceil(count / eq.res.count)
    curr[react] = 0
    for comp in eq.reacts:
        print("  ", comp)
        if comp.name not in curr:
            curr[comp.name] = mult * comp.count
        else:
            curr[comp.name] += mult * comp.count
print(curr)
print(curr["ORE"])