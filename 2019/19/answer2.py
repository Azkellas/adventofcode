import sys
sys.path.append("..")
from Amp import Amp
codes = input()

amp = Amp(codes)

codes = [int(i) for i in codes.split(',')]
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def copy(self):
        return Point(self.x, self.y)

start = Point(6, 5)
print(start.x, start.y)

bottomLine = [start]
while bottomLine[-1].y < 2000:
    test = bottomLine[-1].copy()
    test.x += 1
    test.y += 1
    # amp = Amp(codes)
    amp.reset(codes)
    amp.inputs.append(test.x)
    amp.inputs.append(test.y)
    res = amp.run()
    if res:
        bottomLine.append(test)
    else:
        neigh = bottomLine[-1].copy()
        neigh.x += 1
        bottomLine.append(neigh)

bottomY = []
x = start.x
print(start.x, start.y)
for i in range(start.x):
    bottomY.append(-1)
while True:
    bestY = -1
    for p in bottomLine:
        if p.x > x:
            break
        if p.x == x:
            bestY = max(bestY, p.y)
    if bestY == -1:
        break
    else:
        bottomY.append(bestY)
        x += 1

topLine = [start]
while topLine[-1].x < 2000:
    test = topLine[-1].copy()
    test.x += 1
    amp.reset(codes)
    amp.inputs.append(test.x)
    amp.inputs.append(test.y)
    res = amp.run()
    if res:
        topLine.append(test)
    else:
        neigh = topLine[-1].copy()
        neigh.x += 1
        neigh.y += 1
        topLine.append(neigh)


rightestX = []
for i in range(start.y):
    rightestX.append(-1)
y = start.y
while True:
    bestX = -1
    for p in topLine:
        if p.y > y:
            break
        if p.y == y:
            bestX = max(bestX, p.x)
    if bestX == -1:
        break
    else:
        rightestX.append(bestX)
        y += 1


print(bottomY)
maxX = len(bottomY)
maxY = len(rightestX)

best = Point(-1, -1)
bestScore = 100_000_000
bestLength = -1
for x in range(maxX):
    for y in range(maxY):
        assX = rightestX[y]
        assY = bottomY[x]
        if x <= assX and y <= assY:
            length = min(assX - x + 1, assY - y + 1)
            score = 10000 * x + y
            if length >= 100 and score < bestScore:
                best = Point(x, y)
                bestScore = score
                bestLength = length

print(best.x, best.y, ":", bestScore, bestLength)