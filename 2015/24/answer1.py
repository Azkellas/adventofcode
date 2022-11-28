import sys
import itertools

packages = [int(i) for i in sys.stdin]

packages.sort(reverse=True)

grp_cnt = 3

target_weight = sum(packages) // grp_cnt

nbs = len(packages)

assign = [-1 for i in range(nbs)]


weights = [0 for i in range(grp_cnt)]

min_cnt = 1e30
min_ent = 1e30

# Note: we don't care about other packages, they will be possible

for pkg_cnt in range(0, 10):
    if min_cnt < 100:
        break
    for cmb in itertools.combinations(packages, pkg_cnt):
        if sum(cmb) == target_weight:
            l = len(cmb)
            if l < min_cnt:
                min_cnt = l
            if l == min_cnt:
                e = 1
                for p in cmb:
                    e *= p
                if e < min_ent:
                    print(cmb)
                    print('new ent ', e)
                    min_ent = e

print(min_ent, "with", min_cnt, "packages")