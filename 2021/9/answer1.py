import sys

grid = []

for line in sys.stdin:
    line = line.strip()
    grid.append([int(c) for c in line])

lx = len(grid[0])
ly = len(grid)
score = 0
for x in range(lx):
    for y in range(ly):
        is_minima = True
        v = grid[y][x]
        if y > 0    and grid[y-1][x] <= v: is_minima = False
        if y < ly-1 and grid[y+1][x] <= v: is_minima = False
        if x > 0    and grid[y][x-1] <= v: is_minima = False
        if x < lx-1 and grid[y][x+1] <= v: is_minima = False
        if is_minima: 
            print(f"new minima of {v} at ({y}, {x})")
            score = score + 1 + v
print(score)