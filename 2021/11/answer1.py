import sys

grid = []

for line in sys.stdin:
    grid.append([int(i) for i in line.strip()])

l = len(grid)

score = 0
for step in range(100):
    lighted = set()
    for y in range(l):
        for x in range(l):
            grid[y][x] = grid[y][x] + 1
    while True:
        flashing = set()
        for y in range(l):
            for x in range(l):
                if grid[y][x] > 9:
                    flashing.add((y, x))
                    lighted.add((y, x))
        if len(flashing):
            for (y, x) in flashing:
                score = score + 1
                grid[y][x] = 0
                for dy in [-1, 0, 1]:
                    for dx in [-1, 0, 1]:
                        if (dy != 0 or dx != 0) and 0 <= y+dy < l and 0 <= x+dx < l:
                            if (y+dy, x+dx) not in lighted:
                                grid[y+dy][x+dx] = grid[y+dy][x+dx] + 1
        else:
            break
    
    for y in range(l):
        for x in range(l):
            print(grid[y][x], end="")
        print()
    print()
print(score)
