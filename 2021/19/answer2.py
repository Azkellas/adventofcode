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

lines = []
for line in sys.stdin:
    lines.append(line.strip())

scanners = []
scanpairs = []
scanner_count = 0

def dist(p1, p2):
    return int(sum(map(lambda i, j: abs(i-j), p1, p2)))

def parse_input():
    for i in range(len(lines)):
        line = lines[i]
        m = re.match("--- scanner (\d+) ---", line)

        if m:
            scan_id = int(m.groups()[0])
            scanner = set()
            while True:
                i = i + 1
                if i == len(lines): break
                c = lines[i]
                if c == "": break
                scanner.add(tuple([int(i) for i in c.split(',')]))
            dists = {}
            for c in scanner:
                dists[c] = []
                for d in scanner:
                    dists[c].append(dist(c, d))
                # dists[c] = Counter(dists[c])
                dists[c].sort()
            scanners.append((scanner, dists, {}))
    # print(scanners)

def getpairs():
    for i in range(scanner_count):
        for j in range(i+1, scanner_count):
            scan1 = scanners[i]
            scan2 = scanners[j]
            clique1 = set()
            clique2 = set()
            needed = 12
            tries = len(scan1[ELT])
            for el1 in scan1[ELT]:
                found2 = False
                d1 = scan1[DST][el1]
                if tries < needed:
                    break
                tries = tries - 1
                for el2 in scan2[ELT]:
                    d2 = scan2[DST][el2]
                    common = 0

                    # needed2 = 12
                    # tries2 = ll
                    # for dd1 in d1:
                    #     if tries2 < needed2:
                    #         break
                    #     tries2 = tries2 - 1
                    #     if dd1 in d2:
                    #         needed2 = needed2 - 1
                    #         common = common + 1

                    for dd1 in d1:
                        if dd1 in d2:
                            common = common + 1
                    # if sum((d1 & d2).values()) >= 12:
                    if common >= 12:
                        needed = needed - 1
                        if  found2:
                            print("Found 2 mapping for 1 point")
                        found2 = True
                        clique1.add(el1)
                        clique2.add(el2)
                        root1 = el1
                        root2 = el2
            if len(clique1) > 0:
                # print(i,j,len(clique1))
                scanpairs.append((i, j, clique1, clique2, root1, root2))

def reconstruct():
    matrices = []
    A = np.identity(3)
    factors = [[-1, 1] for i in range(3)]
    for m in itertools.permutations(A):
        for f in itertools.product(*factors):
            matrices.append(np.multiply(m, f))

    reconstructed = copy.deepcopy(scanners[0][ELT])
    seen = set()
    seen.add(0)
    for el in scanners[0][ELT]:
        scanners[0][MAP][el] = el

    orientations = [np.identity(3) for i in range(scanner_count)]
    positions    = [(0,0,0) for i in range(scanner_count)]

    while len(seen) != scanner_count:
        for fm, to, c1, c2, root1, root2 in scanpairs:
            if (fm in seen) != (to in seen):
                if to in seen:
                    fm,to, c1,c2, root1,root2 = to,fm, c2,c1, root2,root1
                seen.add(to)

                # a pair is only used once, no need to move it elsewhere
                ord1 = sorted(list(c1), key=lambda t: dist(root1, t))
                ord2 = sorted(list(c2), key=lambda t: dist(root2, t))

                B = orientations[fm]
                for N in matrices:
                    foundabc = True
                    for k in range(1, len(ord1)):
                        next1 = ord1[k]
                        next2 = ord2[k]

                        delta1 = np.array(list(map(lambda i, j: i - j, next1, root1)))
                        delta2 = np.array(list(map(lambda i, j: i - j, next2, root2)))

                        v = delta2
                        r = delta1

                        if not np.array_equal(np.matmul(N, v), r):
                            foundabc = False
                            break
                    if foundabc:
                        orientations[to] = np.matmul(B, N)
                        break
                if not foundabc:
                    print("error transposition", delta1, delta2)
                
                r1 = np.array(scanners[fm][MAP][root1])
                for el2 in scanners[to][ELT]:
                    new_pos = r1 + np.matmul(orientations[to], np.array(el2) - np.array(root2))
                    new_pos = tuple(new_pos)
                    scanners[to][MAP][el2] = new_pos
                    reconstructed.add(new_pos)

                # scanner position for debugging purposes
                scan_pos = r1 + np.matmul(orientations[to], -np.array(root2))
                # print(f"scanner {to} position = {tuple(scan_pos)}")
                positions[to] = tuple(scan_pos)

    print(f"P1 {len(reconstructed)}")

    mx = 0
    for i in range(scanner_count):
        for j in range(i+1, scanner_count):
            d = dist(positions[i], positions[j])
            if d > mx: mx = d
    print(f"P2 {mx}")


def main():
    global scanners, scanpairs, scanner_count

    scanners = []
    scanpairs = []
    scanner_count = 0

    parse_input()

    scanner_count = len(scanners)

    getpairs()

    reconstruct()

if __name__ == '__main__':
    for i in range(1):
        print(i)
        main()
# r = sorted(list(reconstructed))
# for el in r:
#     print(el)

