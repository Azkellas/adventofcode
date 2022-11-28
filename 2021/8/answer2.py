import sys

from itertools import permutations 

lines = sys.stdin

tot = 0

def validate(cipher, permut, target):
    for c in cipher:
        if permut[ord(c) - ord('a')] not in target:
            return False
    return True

def check_all(cipher, permut):
    if len(cipher) == 5:
        # check against 2, 3, 5:
        if validate(cipher, permut, "acdeg"): return 2
        if validate(cipher, permut, "acdfg"): return 3
        if validate(cipher, permut, "abdfg"): return 5
        return -1
    elif len(cipher) == 6:
        # check against 0, 6, 9
        if validate(cipher, permut, "abcefg"): return 0
        if validate(cipher, permut, "abdefg"): return 6
        if validate(cipher, permut, "abcdfg"): return 9
        return -1
    else:
        # only one check
        if validate(cipher, permut, "cf"):     return 1
        if validate(cipher, permut, "acf"):    return 7
        if validate(cipher, permut, "bcdf"):   return 4
        return 8
        # if len(n) == 2: return 1
        # if len(n) == 3: return 7
        # if len(n) == 4: return 4
        # if len(n) == 7: return 8


score = 0
for line in lines:
    segments = "abcdefg"
    [a, b] = line.split('|')
    nbs = a.split()
    permuts = permutations(segments)
    l = 0
    seg_1 = [n for n in nbs if len(n) == 2][0]
    seg_7 = [n for n in nbs if len(n) == 3][0]
    seg_4 = [n for n in nbs if len(n) == 4][0]
    seg_8 = [n for n in nbs if len(n) == 7][0]
    for p in permuts:
        # if not validate(seg_1, p, "cf"):
        #     continue
        # if not validate(seg_7, p, "acf"):
        #     continue
        # if not validate(seg_4, p, "bcdf"):
        #     continue
        # 8 is always valid

        valid = True
        for n in nbs:
            if check_all(n, p) == -1:
                valid = False
                break
        if not valid:
            continue

        print("Found the answer")

        res = 0
        for n in b.split():
            r = check_all(n, p)
            res = res * 10 + r
        score = score + res

print(score)