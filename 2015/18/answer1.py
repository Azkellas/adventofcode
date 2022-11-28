import sys

grid = []
for line in sys.stdin:
    grid.append([c == '#' for c in line.strip()])

deltas = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]

for step in range(100):
    new_grid = [[False for i in range(100)] for j in range(100)]
    for x in range(100):
        for y in range(100):
            neigh = 0
            for (dx, dy) in deltas:
                if 0 <= x + dx < 100 and 0 <= y + dy < 100:
                    neigh += grid[x + dx][y + dy]
            
            if grid[x][y]:
                new_grid[x][y] = (2 <= neigh <= 3)
            else:
                new_grid[x][y] = neigh == 3
    grid = new_grid


score = 0

for i in range(100):
    for j in range(100):
        score += grid[i][j]


print(score)