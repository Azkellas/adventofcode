from sys import stdin
import math

grid = []

width = -1
height = 0

for line in stdin:
    height += 1
    line = line.strip()
    grid.append(line)
    width = len(line)

rGrid = [[ [] for i in range(height)] for i in range(width)]
for x in range(width):
    for y in range(height):
        rGrid[x][y] = grid[y][x]
grid = rGrid

bestX = -1
bestY = -1
bestScore = -1
for x in range(width):
    for y in range(height):
        if grid[x][y] != '#':
            continue  # cannot put a station here
        
        total = 0
        for dx in range(-width, width):
            for dy in range(-height, height):
                if dx == 0 and dy == 0:
                    continue
                if math.gcd(dx, dy) != 1:
                    continue

                currX = x + dx
                currY = y + dy
                while currX >= 0 and currX < width and currY >= 0 and currY < height:
                    if grid[currX][currY] == '#':
                        total += 1
                        break
                    currX += dx
                    currY += dy
                if total > bestScore:
                    bestScore = total
                    bestX = x
                    bestY = y

print(bestX, bestY, bestScore)

def getAngle(a, b, c):
    ang = math.atan2(c[1]-b[1], c[0]-b[0]) - math.atan2(a[1]-b[1], a[0]-b[0])
    if ang > math.pi:
        ang -= 2*math.pi
    if ang < -math.pi:
        ang += 2*math.pi
    return ang

def getDist(a, b):
    return abs(b[0]-a[0]) + abs(b[1]-a[1])

station = [bestX, bestY]

grid[station[0]][station[1]] = '.'
target = -1
for dy in range(bestY):
    if grid[station[0]][station[1] - dy] == '#':
        target = [station[0], station[1] - dy]
        break

counter = 0
while target != -1:
    counter += 1
    print(counter, ':', target[0], target[1])
    if counter == 200:
        print("SOL =", target[0]*100 + target[1])
    grid[target[0]][target[1]] = '.'

    bestDist = -1
    bestAngle = 1000000
    newTarget = -1
    for x in range(width):
        for y in range(height):
            if grid[x][y] == '#':
                angle = getAngle(target, station, [x, y])
                # print("  |", x, y, angle)
                if angle <= 0:
                    continue
                if angle < bestAngle or (angle == bestAngle and getDist(station, [x, y]) < bestDist):
                    bestAngle = angle
                    bestDist = getDist(station, [x, y])
                    newTarget = [x, y]


    if newTarget == -1:
        for x in range(width):
            for y in range(height):
                if grid[x][y] == '#':
                    angle = getAngle(target, station, [x, y])
                    print("  |", x, y, angle)
                    if angle < bestAngle or (angle == bestAngle and getDist(station, [x, y]) < bestDist):
                        bestAngle = angle
                        bestDist = getDist(station, [x, y])
                        newTarget = [x, y]

    target = newTarget

for y in range(height):
    for x in range(width):
        print(grid[x][y], end="")
    print()

# print(getAngle(curr, base, [curr[0]+1, curr[1]]))
# print(getAngle(curr, base, [curr[0]-1, curr[1]]))

# print(getAngle(curr, base, [curr[0], curr[1]+1]))
# print(getAngle(curr, base, [curr[0], curr[1]-1]))
