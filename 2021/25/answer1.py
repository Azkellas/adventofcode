import sys
import copy

grid = []

for line in sys.stdin:
    grid.append([c for c in line.strip()])

lx, ly = len(grid[0]), len(grid)

steps = 0
while True:
    print(steps)
    # for y in range(ly):
    #     for x in range(lx):
    #         print(grid[y][x], end="")
    #     print()
    # print()
    # if steps > 5: exit()
    steps += 1

    moved = False
    ngrid = [['.' for x in range(lx)] for y in range(ly)]
    # steap eas
    for y in range(ly):
        for x in range(lx):
            if grid[y][x] == '>':
                if grid[y][(x+1)%lx] == '.':
                    moved = True
                    ngrid[y][(x+1)%lx] = '>'
                else:
                    ngrid[y][x] = '>'
            if grid[y][x] == 'v':
                ngrid[y][x] = 'v'
    grid = copy.deepcopy(ngrid)

    ngrid = [['.' for x in range(lx)] for y in range(ly)]
    for y in range(ly):
        for x in range(lx):
            if grid[y][x] == 'v':
                if grid[(y+1)%ly][x] == '.':
                    moved = True
                    ngrid[(y+1)%ly][x] = 'v'
                else:
                    ngrid[y][x] = 'v'
            if grid[y][x] == '>':
                ngrid[y][x] = '>'
    grid = copy.deepcopy(ngrid)

    if not moved:
        print(steps)
        for y in range(ly):
            for x in range(lx):
                print(grid[y][x], end="")
            print()
        print()
        exit()