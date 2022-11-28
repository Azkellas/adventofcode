import sys
import re

X = 0
Y = 1
Z = 2
MIN=1
MAX=2

def intersects(R1, R2):
    return not(R1[1][0]>R2[2][0] or R1[2][0]<R2[1][0] or R1[1][1]>R2[2][1] or R1[2][1]<R2[1][1] or R1[1][2]>R2[2][2] or R1[2][2]<R2[1][2])

def is_inside(R1, R2):
    return R2[1][0] <= R1[1][0] <= R2[2][0] and R2[1][1] <= R1[1][1] <= R2[2][2] and R2[1][2] <= R1[1][2] <= R2[2][2] and \
           R2[1][0] <= R1[2][0] <= R2[2][0] and R2[1][1] <= R1[2][1] <= R2[2][2] and R2[1][2] <= R1[2][2] <= R2[2][2]

cubes = []
for line in sys.stdin:
    # on x=-20..26,y=-36..17,z=-47..7
    m = re.match("(\w+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)", line)
    cmd,x1,x2,y1,y2,z1,z2 = m.groups()
    cubes.append((cmd, (int(x1),int(y1),int(z1)), (int(x2),int(y2),int(z2))))

ncubes = set()

def intersect_false(n, c):
    if c[MAX][Y] < n[MAX][Y]:
        e1, e2 = (n[MIN][X], c[MAX][Y] + 1, n[MIN][Z]), n[MAX]
        ncubes.add(("on", e1, e2))
    if c[MIN][Y] > n[MIN][Y]:
        e1, e2 = n[MIN], (n[MAX][X], c[MIN][Y] - 1, n[MAX][Z])
        ncubes.add(("on", e1, e2))

    ymin, ymax = max(n[MIN][Y], c[MIN][Y]), min(n[MAX][Y], c[MAX][Y])
    if c[MAX][X] < n[MAX][X]:
        e1, e2 = (c[MAX][X] + 1, ymin, n[MIN][Z]), (n[MAX][X], ymax, n[MAX][Z])
        ncubes.add(("on", e1, e2))
    if c[MIN][X] > n[MIN][X]:
        e1, e2 = (n[MIN][X], ymin, n[MIN][Z]), (c[MIN][X] - 1, ymax, n[MAX][Z])
        ncubes.add(("on", e1, e2))

    xmin, xmax = max(n[MIN][X], c[MIN][X]), min(n[MAX][X], c[MAX][X])
    if c[MAX][Z] < n[MAX][Z]:
        e1, e2 = (xmin, ymin, c[MAX][Z] + 1), (xmax, ymax, n[MAX][Z])
        ncubes.add(("on", e1, e2))
    if c[MIN][Z] > n[MIN][Z]:
        e1, e2 = (xmin, ymin, n[MIN][Z]), (xmax, ymax, c[MIN][Z] - 1)
        ncubes.add(("on", e1, e2))

for c in cubes:
    ncopy = ncubes.copy()
    for n in ncopy:
        if intersects(c, n):
            ncubes.remove(n)
            intersect_false(n, c)

    if c[0] == "on":
        ncubes.add(c)

print("lcubes", len(ncubes))
total = 0
for c in ncubes:
    # print(c)
    total = total + (1+c[MAX][X]-c[MIN][X])*(1+c[MAX][Y]-c[MIN][Y])*(1+c[MAX][Z]-c[MIN][Z])
print(total)
