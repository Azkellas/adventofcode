import sys
import copy 

grid = set()

code = input().strip()

input()
print(len(code))


y = 0
for line in sys.stdin:
    line = line.strip()
    for x in range(len(line)):
        if line[x] == '#':
            grid.add((x, y))
    y = y + 1
# print(grid)

xmin,xmax = 1_000_000, -1_000_000
ymin,ymax = 1_000_000, -1_000_000

border_idx = 0
border_color = '.'

def is_light(x, y):
    if not xmin <= x <= xmax: return border_color == "#"
    if not ymin <= y <= ymax: return border_color == "#"
    return (x, y) in grid

for step in range(2):
    xmin,xmax = 1_000_000, -1_000_000
    ymin,ymax = 1_000_000, -1_000_000
    newgrid = set()
    for x,y in grid:
        if x < xmin: xmin = x
        if x > xmax: xmax = x
        if y < ymin: ymin = y
        if y > ymax: ymax = y
    

    for y in range(ymin - 1, ymax + 5):
        for x in range(xmin - 1, xmax + 10):
            c = ""
            c = c + ["0","1"][is_light(x-1, y-1)]
            c = c + ["0","1"][is_light(x,   y-1)]
            c = c + ["0","1"][is_light(x+1, y-1)]

            c = c + ["0","1"][is_light(x-1, y)]
            c = c + ["0","1"][is_light(x,   y)]
            c = c + ["0","1"][is_light(x+1, y)]

            c = c + ["0","1"][is_light(x-1, y+1)]
            c = c + ["0","1"][is_light(x,   y+1)]
            c = c + ["0","1"][is_light(x+1, y+1)]

            # print(x, y, c, int(c, 2))
            c = int(c, 2)
            if code[c] == "#": newgrid.add((x, y))
    grid = copy.deepcopy(newgrid)

    score = 0
    xmin,xmax = 1_000_000, -1_000_000
    ymin,ymax = 1_000_000, -1_000_000
    for x,y in grid:
        if x < xmin: xmin = x
        if x > xmax: xmax = x
        if y < ymin: ymin = y
        if y > ymax: ymax = y
    for x,y in grid:
        if xmin < x < xmax and ymin < y < ymax:
            score = score + 1

    border_color = code[border_idx]
    border_idx = [0, 511][border_color == '#']

    print(step+1, ":", "grid", len(grid))
