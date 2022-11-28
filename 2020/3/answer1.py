import sys

grid = []
for line in sys.stdin:
    grid.append(line[:-1])


x, y = 0, 0
height, width = len(grid), len(grid[0])
dx, dy = 3, 1

trees = 0
while y < height:
    trees += (grid[y][x] == '#')
    x += dx
    y += dy
    if x >= width:
        x %= width
print(trees)


