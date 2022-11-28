import sys


input_grid = []
for line in sys.stdin:
    input_grid.append(list(line.strip()))


input_x = len(input_grid[0])
input_y = len(input_grid)


step_count = 6
size_x = input_x + 2 * step_count
size_y = input_y + 2 * step_count
size_z =       1 + 2 * step_count
size_w =       1 + 2 * step_count

m_x = size_x // 2
m_y = size_y // 2
m_z = size_z // 2
m_w = size_z // 2

grid = [[[['.' for w in range(size_w)] for z in range(size_z)] for y in range(size_y)] for x in range(size_x)]

for x in range(input_x):
    for y in range(input_y):
        grid[step_count + x][step_count + y][step_count][step_count] = input_grid[y][x]

for step in range(step_count):
    new_grid = [[[['.' for w in range(size_w)] for z in range(size_z)] for y in range(size_y)] for x in range(size_x)]

    for x in range(size_x):
        for y in range(size_y):
            for z in range(size_z):
                for w in range(size_w):
                    neigh = 0
                    # print(x - step_count, y - step_count, z - step_count)
                    for dx in [-1, 0, 1]:
                        for dy in [-1, 0, 1]:
                            for dz in [-1, 0, 1]:
                                for dw in [-1, 0, 1]:
                                    n_x, n_y, n_z, n_w = x + dx, y + dy, z + dz, w + dw
                                    if (dx != 0 or dy != 0 or dz != 0 or dw != 0) \
                                        and 0 <= n_x < size_x and 0 <= n_y < size_y and 0 <= n_z < size_z and 0 <= n_w < size_w:
                                        # print('  ', n_x, n_y, n_z, grid[n_x][n_y][n_z])
                                        neigh += (grid[n_x][n_y][n_z][n_w] == '#')
                    # print(grid)
                    valid = grid[x][y][z][w] == '#' and 2 <= neigh <= 3 \
                        or  grid[x][y][z][w] == '.' and      neigh == 3
                    new_grid[x][y][z][w] = ['.', '#'][valid]
    
    grid = new_grid

    print('step:', step)

tot = 0
for z in range(size_z):
    for x in range(size_x):
        for y in range(size_y):
            for w in range(size_w):
                tot += grid[x][y][z][w] == '#'
print(tot)