import sys

grid = []
for line in sys.stdin:
    grid.append(line.strip())

width  = len(grid[0])
height = len(grid)

changed = True
steps = 0
while changed:
    steps += 1
    changed = False
    new_grid = [['.' for i in range(width)] for j in range(height)]

    for x in range(width):
        for y in range(height):
            if grid[y][x] != '.':
                neigh = 0

                for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]:
                    if 0 <= x + dx < width and 0 <= y + dy < height:
                        neigh += (grid[y+dy][x+dx] == '#')
                
                if grid[y][x] == '#':
                    new_grid[y][x] = ['#', 'L'][neigh >= 4]

                if grid[y][x] == 'L':
                    new_grid[y][x] = ['L', '#'][neigh == 0]

                if grid[y][x] != new_grid[y][x]:
                    changed = True
    
    grid = new_grid



tot = 0
for x in range(width):
    for y in range(height):
        tot += grid[y][x] == '#'


print(tot, 'in', steps)

