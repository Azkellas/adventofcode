import sys
import re


def is_in(p, rec):
    return rec[1][0] <= p[0] <= rec[2][0] and rec[1][1] <= p[1] <= rec[2][1] and rec[1][2] <= p[2] <= rec[2][2]

def is_init(rec):
    for i in range(2):
        for j in range(3):
            if abs(rec[i+1][j]) > 50: return False
    return True

# on x=-20..26,y=-36..17,z=-47..7

cubes = []

for line in sys.stdin:
    m = re.match("(\w+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)", line)
    cmd,x1,x2,y1,y2,z1,z2 = m.groups()
    cubes.append((cmd, (int(x1),int(y1),int(z1)), (int(x2),int(y2),int(z2))))

light = 0
for x in range(-50, 51):
    for y in range(-50, 51):
        for z in range(-50, 51):
            l = False
            for c in cubes:
                if is_init(c) and is_in((x,y,z), c):
                    l = (c[0] == "on")
            if l: light = light + 1
print(light)