import sys
import numpy as np

grid = []

for line in sys.stdin:
    grid.append([int(i) for i in line.strip()])

l = len(grid)

score = 0
step = 0
while True:
    seen = 0
    step = step + 1
    for y, x in np.ndindex((l, l)):
        grid[y][x] = grid[y][x] + 1
    while True:
        flashing = set()
        for y in range(l):
            for x in range(l):
                if grid[y][x] > 9:
                    flashing.add((y, x))
                    grid[y][x] = 0
        if len(flashing) == 0:
            break
        for (y, x) in flashing:
            seen = seen + 1
            for dx, dy in np.ndindex((3, 3)):
                dx, dy = np.subtract((dx, dy), (1, 1))
                if all(0 <= a < l for a in (y+dy, x+dx)):
                    if grid[y+dy][x+dx] > 0:
                        grid[y+dy][x+dx] = grid[y+dy][x+dx] + 1
    
    if seen == l*l:
        print(f"at step {step}")
        exit(0)
