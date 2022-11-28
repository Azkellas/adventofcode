from sys import stdin

grid = [ ['.' for i in range(5)] for j in range(5)]
newGrid = [ ['.' for i in range(5)] for j in range(5)]

y = -1
for line in stdin:
    y += 1
    for x in range(5):
        grid[x][y] = line[x]

scores = {}
while True:
    for y in range(5):
        for x in range(5):
            print(grid[x][y], end="")
        print()

    score = 0
    for x in range(5):
        for y in range(5):
            if grid[x][y] == '#':
                score += pow(2, x + 5*y)
    if score in scores:
        print("double score =", score)
        exit()
    scores[score] = 1
    print(score)
    print()

    for x in range(5):
        for y in range(5):
            neigh = 0
            for p in [[-1, 0],
                      [0, -1], [0, 1],
                      [1, 0]]:
                nX = x + p[0]
                nY = y + p[1]
                if 0 <= nX < 5 and 0 <= nY < 5 and grid[nX][nY] == '#':
                    neigh += 1
            # print(x, y,neigh)
            if grid[x][y] == '#':
                if neigh == 1:
                    newGrid[x][y] = '#'
                else:
                    newGrid[x][y] = '.'
            else:
                if neigh == 1 or neigh == 2:
                    newGrid[x][y] = '#'
                else:
                    newGrid[x][y] = '.'
    
    for x in range(5):
        for y in range(5):
            grid[x][y] = newGrid[x][y]