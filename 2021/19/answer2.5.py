import re
import sys
from collections import Counter
import math

import itertools
import numpy as np
import copy

ELT = 0
DST = 1
MAP = 2

elt_count = 0

elts = [np.array([0,0,0]) for i in range(1000)]
dists = [None for i in range(10000)]
maps = [None for i in range(10000)]

def dist(p1, p2):
    return int(sum(map(lambda i, j: abs(i-j), p1, p2)))

scanners = []
for line in sys.stdin:
    line = line.strip()
    m = re.match("--- scanner (\d+) ---", line)
    if m:
        scan_id = int(m.groups()[0])
        # print(f"new scanner: {scan_id}")
        scanner = set()
        for c in sys.stdin:
            c = c.strip()
            if c == "": break
            elts[elt_count] = np.array([int(i) for i in c.split(',')])
            scanner.add(elt_count)
            elt_count = elt_count + 1
        for eid in scanner:
            dsts = []
            for eid2 in scanner:
                dsts.append(dist(elts[eid], elts[eid2]))
            dists[eid] = Counter(dsts)
        scanners.append(scanner)
# print(scanners)
elts = elts[:elt_count]
dists = dists[:elt_count]
maps = maps[:elt_count]

scanner_count = len(scanners)

scanpairs = []
for i in range(scanner_count):
    for j in range(i+1, scanner_count):
        scan1 = scanners[i]
        scan2 = scanners[j]
        clique1 = set()
        clique2 = set()
        for el1_id in scan1:
            found2 = False
            d1 = dists[el1_id]
            for el2_id in scan2:
                d2 = dists[el2_id]
                if sum((d1 & d2).values()) >= 12:
                # if len(list((d1 & d2).elements())) >= 12:
                    if  found2:
                        print("Found 2 mapping for 1 point")
                    found2 = True
                    clique1.add(el1_id)
                    clique2.add(el2_id)
                    root1 = el1_id
                    root2 = el2_id
        if len(clique1) > 0:
            # print(i,j,len(clique1))
            scanpairs.append((i, j, clique1, clique2, root1, root2))

reconstructed = set()
for eid in scanners[0]:
    reconstructed.add(tuple(elts[eid]))

seen = set()
seen.add(0)

exit()
for el in scanners[0]:
    maps[el] = elts[eid]

orientations = [np.identity(3) for i in range(scanner_count)]
positions    = [(0,0,0) for i in range(scanner_count)]

while len(seen) != scanner_count:
    for i, j, ci, cj, ri, rj in scanpairs:
        if i in seen and j not in seen or j in seen and i not in seen:
            if i in seen:
                fm, to, c1, c2, root1, root2 = i,j, ci,cj, ri,rj
            else:
                fm, to, c1, c2, root1, root2 = j,i, cj,ci, rj,ri
            # print(f'add {to} next to {fm}')
            seen.add(to)

            ord1 = sorted(list(c1), key=lambda t: dist(root1, t))
            ord2 = sorted(list(c2), key=lambda t: dist(root2, t))

            next1 = ord1[3]
            next2 = ord2[3]

            delta1 = np.array(list(map(lambda i, j: i - j, next1, root1)))
            delta2 = np.array(list(map(lambda i, j: i - j, next2, root2)))

            foundabc = False

            B = orientations[fm]
            A = np.identity(3)
            for m in itertools.permutations(A):
                M = m
                v = delta2
                r = delta1
                # print(r)

                for da in [-1,1]:
                    for db in [-1,1]:
                        for dc in [-1,1]:
                            if foundabc: break
                            N = np.multiply(M, [da,db,dc])
                            # print(N)
                            if np.array_equal(np.matmul(N, v), r):
                                orientations[to] = np.matmul(B, N)
                                foundabc = True
                                break
            if not foundabc:
                print("error transposition", delta1, delta2)
            
            r1 = np.array(scanners[fm][MAP][root1])
            for el2 in scanners[to][ELT]:
                new_pos = r1 + np.matmul(orientations[to], np.array(el2) - np.array(root2))
                new_pos = tuple(new_pos)
                scanners[to][MAP][el2] = new_pos
                reconstructed.add(new_pos)

            # scanner position
            scan_pos = r1 + np.matmul(orientations[to], -np.array(root2))
            positions[to] = tuple(scan_pos)



# r = sorted(list(reconstructed))
# for el in r:
#     print(el)
print(f"P1 {len(reconstructed)}")


mx = 0
for i in range(scanner_count):
    for j in range(i+1, scanner_count):
        d = dist(positions[i], positions[j])
        if d > mx: mx = d
print(f"P2 {mx}")
