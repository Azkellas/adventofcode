import sys

grid = {}

for line in sys.stdin:
    x, y, z = 0, 0, 0
    line = line.strip()
    while line != '':
        if line.startswith('e'):
            x += 1
            y -= 1
            line = line[1:]
        if line.startswith('se'):
            z += 1
            y -= 1
            line = line[2:]
        if line.startswith('sw'):
            z += 1
            x -= 1
            line = line[2:]
        if line.startswith('w'):
            y += 1
            x -= 1
            line = line[1:]
        if line.startswith('nw'):
            y += 1
            z -= 1
            line = line[2:]
        if line.startswith('ne'):
            x += 1
            z -= 1
            line = line[2:]
    if (x, y, z) in grid:
        grid[(x, y, z)] = not grid[(x, y, z)]
    else:
        grid[(x, y, z)] = True

new_grid = set()
for tile in grid:
    if grid[tile]:
        new_grid.add(tile)
grid = new_grid

deltas = [-1, 0, 1]

print('p1:', len(grid))

print(grid)
for day in range(100):
    neighbours = {}
    for tile in grid:
        for dx in deltas:
            for dy in deltas:
                dz = -dx-dy
                if (dx == 0 and dy*dz != 0 and dy+dz == 0) \
                or (dy == 0 and dx*dz != 0 and dx+dz == 0) \
                or (dz == 0 and dx*dy != 0 and dx+dy == 0):
                    (x, y, z) = (tile[0] + dx, tile[1] + dy, tile[2] + dz)
                    if (x, y, z) in neighbours:
                        neighbours[(x, y, z)] += 1
                    else:
                        neighbours[(x, y, z)] = 1


    # print(len(neighbours))
    if len(grid) * 6 != sum(neighbours.values()):
        print('======================================Error')
        exit()

    new_grid = set()
    for tile in neighbours:
        if tile in grid and 1 <= neighbours[tile] <= 2:
            # print('old black')
            new_grid.add(tile)
        if not tile in grid and neighbours[tile] == 2:
            # print('new black')
            new_grid.add(tile)
    
    grid = new_grid
    
    print(day+1, len(grid))
    # print(grid)
# 4 bag trad