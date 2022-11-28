import sys
import re

lines = sys.stdin
reg = "(?P<y1>\d+),(?P<x1>\d+) -> (?P<y2>\d+),(?P<x2>\d+)"

map = [[0 for i in range(1000)] for j in range(1000)]
max_diag = 0
for line in lines:
    m = re.match(reg, line).groupdict()

    for k in m:
        m[k] = int(m[k])
        if m[k] >= max_diag:
            max_diag = m[k] + 1

    if m["x1"] == m["x2"]:
        x = m["x1"]
        y1, y2 = min(m["y1"], m["y2"]), max(m["y1"], m["y2"])
        for y in range(y1, y2 + 1):
            map[x][y] = map[x][y] + 1

    elif m["y1"] == m["y2"]:
        y = m["y1"]
        x1, x2 = min(m["x1"], m["x2"]), max(m["x1"], m["x2"])
        for x in range(x1, x2 + 1):
            map[x][y] = map[x][y] + 1

    else:
        continue
        # diagonal
        x1, y1, x2, y2 = m["x1"], m["y1"], m["x2"], m["y2"]
        if abs(x1 - x2) == abs(y1 - y2):
            xpos = x2 > x1
            ypos = y2 > y1
            delta = abs(x1 - x2)
            for d in range(delta + 1):
                x = x1 + [-d,d][xpos]
                y = y1 + [-d,d][ypos]
                map[x][y] = map[x][y] + 1



score = 0
for i in range(max_diag):
    for j in range(max_diag):
        print(map[i][j] if map[i][j] > 0 else '.', end=" ")
        if map[i][j] >= 2:
            score += 1
    print()

print(score)