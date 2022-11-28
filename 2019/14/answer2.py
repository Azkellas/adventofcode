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
depths = {}
depths['ORE'] = 0

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


curr = {}
curr['FUEL'] = 1
while True:
    minDepth = -1
    for name in curr:
        if curr[name] > 0 and name != "ORE" and depths[name] > minDepth:
            react = name
            minDepth = depths[name]

    if minDepth == -1:
        break

    # print("reaction for ", react)
    eq = eqs[react]
    count = curr[react]
    mult = math.ceil(count / eq.res.count)
    curr[react] -= mult * eq.res.count
    for comp in eq.reacts:
        # print("  ", comp)
        if comp.name not in curr:
            curr[comp.name] = mult * comp.count
        else:
            curr[comp.name] += mult * comp.count

oreNeededFor1 = curr["ORE"]


mini = 1000000000000 // oreNeededFor1
mini += 550000
fuelProduced = mini
print("mini", mini)
curr = {}
curr["FUEL"] = mini
while True:
    while True:
        minDepth = -1
        for name in curr:
            if curr[name] > 0 and name != "ORE" and depths[name] > minDepth:
                react = name
                minDepth = depths[name]

        if minDepth == -1:
            break

        # print("reaction for ", react)
        eq = eqs[react]
        count = curr[react]
        mult = math.ceil(count / eq.res.count)
        curr[react] -= mult * eq.res.count
        for comp in eq.reacts:
            # print("  ", comp)
            if comp.name not in curr:
                curr[comp.name] = mult * comp.count
            else:
                curr[comp.name] += mult * comp.count
    print("Curr ore: ", curr["ORE"], fuelProduced)
    if curr["ORE"] > 1000000000000:
        break
    else:
        curr["FUEL"] += 1
        fuelProduced += 1

print(curr)
print(curr["ORE"])
print(fuelProduced - 1)

# 5*O = 5*A
# 2*A = 2*B
# A + B = F
# O:0, A:0, B:0, F:1
# O:0, A:1, B:1, F:0
# O:0, A:3, B:-1, F:0
# O:5, A:-2, B:-1, F:0