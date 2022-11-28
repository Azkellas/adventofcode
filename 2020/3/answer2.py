nimport sys

grid = []
for line in sys.stdin:
    grid.append(line[:-1])


height = len(grid)
width = len(grid[0])


deltas = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]

total_trees = 1
fr delta in deltas:
    x, y = 0, 0
    dx, dy = delta[0], delta[1]
    trees = 0
    while y < height:
        trees += (grid[y][x] == '#')
        x += dx
        y += dy
        if x >= width:
            x = x % width
    total_trees *= trees

print(total_trees)