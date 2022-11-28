from sys import stdin

smallGrid = [ ['.' for i in range(5)] for j in range(5)]

grid = [[ ['.' for i in range(5)] for j in range(5)] for i in range(500)]
newGrid = [[ ['.' for i in range(5)] for j in range(5)] for i in range(500)]

base = 250

y = -1
for line in stdin:
    y += 1
    for x in range(5):
            grid[base][x][y] = line[x]

for step in range(200):
    print(step)
    for depth in range(500):
        for x in range(5):
            for y in range(5):
                if x == 2 and y == 2:
                    continue
                neigh = 0
                for p in [[-1, 0],
                        [0, -1], [0, 1],
                        [1, 0]]:
                    nX = x + p[0]
                    nY = y + p[1]
                    if 0 <= nX < 5 and 0 <= nY < 5 and (nX != 2 or nY != 2):
                        if grid[depth][nX][nY] == '#':
                            neigh += 1
                    else:
                        # recursive neighbour
                        if x == 0 or y == 0 or x == 4 or y == 4:
                            # neighbour at depth - 1
                            if nX < 0:
                                nX = 1
                            elif nX == 5:
                                nX = 3
                            else:
                                nX = 2
                            if nY < 0:
                                nY = 1
                            elif nY == 5:
                                nY = 3
                            else:
                                nY = 2
                            if depth != 0 and grid[depth-1][nX][nY] == '#':
                                neigh += 1
                        elif nX == 2 and nY == 2:
                            # neighbours at depth + 1
                            if y != 2:
                                lineY = [0, 4][y == 3]
                                for oX in range(5):
                                    if depth != 499 and grid[depth+1][oX][lineY] == '#':
                                        neigh += 1
                            if x != 2:
                                colX = [0, 4][x == 3]
                                for oY in range(5):
                                    if depth != 499 and grid[depth+1][colX][oY] == '#':
                                        neigh += 1
                        else:
                            print("weird", x, y, nX, nY)


                # print(x, y,neigh)
                if grid[depth][x][y] == '#':
                    if neigh == 1:
                        newGrid[depth][x][y] = '#'
                    else:
                        newGrid[depth][x][y] = '.'
                else:
                    if neigh == 1 or neigh == 2:
                        newGrid[depth][x][y] = '#'
                    else:
                        newGrid[depth][x][y] = '.'
        
    for depth in range(500):
        for x in range(5):
            for y in range(5):
                grid[depth][x][y] = newGrid[depth][x][y]


for d in range(500):
    ds = 0
    for x in range(5):
        for y in range(5):
            if x == 2 and y == 2:
                continue
            if grid[d][x][y] == '#':
                ds += 1
    if ds:
        print("depth", d - base)
        for y in range(5):
            for x in range(5):
                if x == 2 and y == 2:
                    print(" ", end="")
                else:
                    print(grid[d][x][y], end="")
            print()
        print()

score = 0
for depth in range(500):
    for x in range(5):
        for y in range(5):
            if x == 2 and y == 2:
                continue
            if grid[depth][x][y] == '#':
                score += 1
print("total bugs", score)