import sys

grid = {}

for line in sys.stdin:
    x, y, z = 0, 0, 0
    line = line.strip()
    while line != '':
        print(line)
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

print(sum(grid.values()))