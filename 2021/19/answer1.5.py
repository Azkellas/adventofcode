import re
import sys
from collections import Counter
import math

import itertools
import numpy as np
import copy

def I(n):
    A = []
    for i in range(n):
        A.append([1 if j == i else 0 for j in range(n)])
    return A

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

reconstructed = copy.deepcopy(scanners[0][1])
seen = set()
seen.add(0)
for el in scanners[0][1]:
    scanners[0][3][el] = el

orientations = [I(3) for i in range(scanner_count)]
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

            next1 = ord1[3]
            next2 = ord2[3]

            delta1 = tuple(map(lambda i, j: i - j, next1, root1))
            delta2 = tuple(map(lambda i, j: i - j, next2, root2))

            foundabc = False

            B = orientations[fm]
            A = I(3)
            for m in itertools.permutations(A):
                M = np.array(m)
                v = np.array(delta2)
                r = np.array(delta1)
                # print(r)

                for da in [-1,1]:
                    for db in [-1,1]:
                        for dc in [-1,1]:
                            if foundabc: break
                            N = np.multiply(M, [da,db,dc])
                            # print(N)
                            if np.array_equal(np.matmul(N, v), r):
                                print("found transpo")
                                orientations[to] = np.matmul(B, N)
                                foundabc = True
                                break
            if not foundabc:
                print("error transposition", delta1, delta2)
            
            r1 = np.array(scanners[fm][3][root1])
            for el2 in scanners[to][1]:
                new_pos = r1 + np.matmul(orientations[to], np.array(el2) - np.array(root2))
                new_pos = tuple(new_pos)
                scanners[to][3][el2] = new_pos
                reconstructed.add(new_pos)

            # scanner position for debugging purposes
            scan_pos = r1 + np.matmul(orientations[to], -np.array(root2))
            print(f"scanner {to} position = {tuple(scan_pos)}")


# r = sorted(list(reconstructed))
# for el in r:
#     print(el)
print(len(reconstructed))
