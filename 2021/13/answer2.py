import sys
import re

points = set()

folds = False
for line in sys.stdin:
    if line.strip() == "":
        folds = True
        continue

    if folds:
        reg = "fold along ([xy])=(\d+)"
        m = re.match(reg, line.strip())
        print(m.groups())
        axis = m.group(1)
        coord = int(m.group(2))
        nset = set()
        for (a, b) in points:
            nset.add((a, b))
            if axis == "x" and a > coord:
                nset.remove((a, b))
                nset.add((2*coord - a, b))
            if axis == "y" and b > coord:
                nset.remove((a, b))
                nset.add((a, 2*coord - b))
        points = nset
        print(len(points))
    else:
        a, b = [int(i) for i in line.strip().split(',')]
        points.add((a, b))

mx, my = 0, 0
for (a, b) in points:
    mx = max(mx, a + 1)
    my = max(my, b + 1)

for y in range(my):
    for x in range(mx):
        if (x, y) in points:
            print('#', end="")
        else:
            print(' ', end="")
    print()
