import re
import sys
from collections import Counter
import math


def dist(p1, p2):
    r = 0
    for i in range(len(p1)):
        r = r + abs(p1[i] - p2[i])
    return r

scanners = []
for line in sys.stdin:
    line = line.strip()
    m = re.match("--- scanner (\d+) ---", line)
    if m:
        scanner = set()
        print("new scanner ", m.groups()[0])
        for c in sys.stdin:
            c = c.strip()
            if c == "": break
            scanner.add(tuple([int(i) for i in c.split(',')]))
        dists = {}
        for c in scanner:
            dists[c] = {}
            for d in scanner:
                dists[c][d] = dist(c, d)
        scanners.append((int(m.groups()[0]), scanner, dists, {}))
# print(scanners)

scanner_count = len(scanners)

scanpairs = []
for i in range(scanner_count):
    for j in range(i+1, scanner_count):
        scan1 = scanners[i]
        scan2 = scanners[j]
        clique1 = set()
        clique2 = set()
        for el1 in scan1[1]:
            found2 = False
            d1 = Counter(scan1[2][el1].values())
            for el2 in scan2[1]:
                d2 = Counter(scan2[2][el2].values())
                if len(list((Counter(d1) & Counter(d2)).elements())) >= 12:
                    if  found2:
                        print("Found 2 mapping for 1 point")
                    found2 = True
                    clique1.add(el1)
                    clique2.add(el2)
                    root1 = el1
                    root2 = el2
        if len(clique1) > 0:
            print(i,j,len(clique1))
            scanpairs.append((i, j, clique1, clique2, root1, root2))

# for i,j,ci,cj,ri,rj in scanpairs:
#     print(i,j)
#     print("   ", ci)
#     print("   ", cj)
#     print("   ", ri,rj)

reconstructed = scanners[0][1]
seen = set()
seen.add(0)
for el in scanners[0][1]:
    scanners[0][3][el] = el

orientations = [(1,2,3) for i in range(scanner_count)]
positions    = [(0,0,0) for i in range(scanner_count)]

while len(seen) != scanner_count:
    for (i, j, ci, cj, ri, rj) in scanpairs:
        if i in seen and j not in seen or j in seen and i not in seen:
            if i in seen:
                fm, to, c1, c2, root1, root2 = i,j, ci,cj, ri,rj
            else:
                fm, to, c1, c2, root1, root2 = j,i, cj,ci, rj,ri
            print(f'add {to} next to {fm}')
            seen.add(to)
            pos_fm = positions[fm]
            ori_fm = orientations[fm]
            ord1 = sorted(list(c1), key=lambda t: dist(root1, t))
            ord2 = sorted(list(c2), key=lambda t: dist(root2, t))

            next1 = ord1[1]
            next2 = ord2[1]

            delta1 = tuple(map(lambda i, j: i - j, next1, root1))
            delta2 = tuple(map(lambda i, j: i - j, next2, root2))

            foundabc = False
            for pa,pb,pc in [(1,2,3), (1,3,2), (2,1,3), (2,3,1), (3,1,2), (3,2,1)]:
                pp = [pa,pb,pc]
                for da in [-1,1]:
                    for db in [-1,1]:
                        for dc in [-1,1]:
                            dd = [da,db,dc]
                            if foundabc: break
                            valid = True
                            for k in range(len(delta1)):
                                if delta1[k] != delta2[pp[k]-1] * dd[k]:
                                    valid = False
                                    break
                            if valid:
                                # print("found orientation:", da*pa, db*pb, dc*pc)
                                foundabc = True
                                orientations[to] = (da*pa, db*pb, dc*pc)
                                break
            if not foundabc:
                print("error transposition", delta1, delta2)
            
            ori_to = orientations[to]
            transfo = [1,2,3]
            for k in range(len(ori_to)):
                # lol
                transfo[k] = int(math.copysign(1, ori_fm[k]) * ori_to[abs(ori_fm[k])-1])
                # transfo[k] = int(math.copysign(1, ori_to[k]) * ori_fm[abs(ori_to[k])-1])
            transfo = tuple(transfo)
            orientations[to] = transfo
            # print(ori_fm, ori_to, transfo)

            r1 = scanners[fm][3][root1]
            for el2 in scanners[to][1]:
                new_pos = [0,0,0]
                for k in range(len(el2)):
                    si = math.copysign(1, transfo[k])
                    ti = abs(transfo[k])-1
                    new_pos[k] = int(r1[k] + si * (el2[ti] - root2[ti]))
                new_pos = tuple(new_pos)
                scanners[to][3][el2] = new_pos
                reconstructed.add(new_pos)

            # scanner position for debugging purposes
            scan_pos = [0,0,0]
            for k in range(len(root2)):
                si = math.copysign(1, transfo[k])
                ti = abs(transfo[k])-1
                scan_pos[k] = int(r1[k] + si * (-root2[ti]))
            print(f"scanner {to} position = {tuple(scan_pos)} with rotation {transfo}")


# r = sorted(list(reconstructed))
# for el in r:
#     print(el)
print(len(reconstructed))