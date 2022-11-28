import sys
sys.path.append("..")
from Amp import Amp
codes = input()
amp = Amp(codes)


class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __add__(self, q):
        return Point(self.x + q.x, self.y + q.y)

    def __str__(self):
        return f"({self.x}, {self.y})"

    def isInside(self):
        return self.x >= 0 and self.x < width and self.y >= 0 and self.y < height


grid = [[]]
while True:
    res = amp.run()
    if res is None:
        break
    print(res,end=", ")
    res = chr(res)
    if res == '\n':
        if len(grid[-1]) == 0:
            grid = grid[:-1]
            break
        print()
        grid.append([])
    else:
        grid[-1].append(res)


print(grid)
height = len(grid)
width = len(grid[0])
print("\n\n",height, width)
for y in range(height):
    for x in range(width):
        if grid[y][x] == '.':
            print(' ', end="")
        else:
            print(grid[y][x], end="")
    print()

print()
tot = 0

dirs = [Point(-1, 0), Point(0, -1), Point(1, 0), Point(0, 1)]

for y in range(height):
    for x in range(width):
        if grid[y][x] != '#':
            continue
        neigh = 0
        for d in dirs:
            n  = Point(x, y) + d
            if n.isInside() and grid[n.y][n.x] == '#':
                neigh += 1
        if neigh > 2:
            print("yx", y, x)
            tot += y * x
print("tot", tot)
