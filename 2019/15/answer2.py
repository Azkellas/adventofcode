import sys
sys.path.append("..")
from Amp import Amp
import time
codes = input()
amp = Amp(codes)
w = 50

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __add__(self, q):
        return Point(self.x + q.x, self.y + q.y)

    def __str__(self):
        return f"({self.x}, {self.y})"

    def isInside(self):
        return self.x >= 0 and self.x < w and self.y >= 0 and self.y < w

dirs = [Point(0, -1), Point(0, 1), Point(-1, 0), Point(1, 0)]

grid = [ ['?' for i in range(w)] for j in range(w)]
depths = [ [0 for i in range(w)] for j in range(w)]

# ugly way to have a 4dim array
nexts = [ [ [ [0 for l in range(w)] for k in range(w)] for i in range(w)] for j in range(w)]

# init nexts
for i in range(w):
    for j in range(w):
        for k in range(w):
            for l in range(w):
                nexts[i][j][k][l] = 0
        for di in range(len(dirs)):
            d = dirs[di]
            n = Point(i, j) + d
            if n.isInside():
                nexts[i][j][n.x][n.y] = di + 1

def bfs(start):
    for i in range(w):
        for j in range(w):
            depths[i][j] = 1000

    depths[start.x][start.y] = 0
    queue = [start]
    while len(queue):
        p = queue.pop(0)
        # print(f"pq {p}")
        for d in dirs:
            n = p + d
            if not n.isInside():
                continue
            # print(f"n {n} {grid[n.x][n.y]}")
            if grid[n.x][n.y] !=  '#' and depths[n.x][n.y] > depths[p.x][p.y] + 1:
                depths[n.x][n.y] = depths[p.x][p.y] + 1
                if p.x != start.x or p.y != start.y:
                    nexts[start.x][start.y][n.x][n.y] = nexts[start.x][start.y][p.x][p.y]
                if grid[n.x][n.y] == ' ':
                    queue.append(n)
            if grid[n.x][n.y] == '?':
                # print(f"goto {n}, dir {nexts[start.x][start.y][n.x][n.y]}")
                return nexts[start.x][start.y][n.x][n.y]

def dist(start, end):
    for i in range(w):
        for j in range(w):
            depths[i][j] = 1000

    depths[start.x][start.y] = 0
    queue = [start]
    while len(queue):
        p = queue.pop(0)
        # print(f"pq {p}")
        for d in dirs:
            n = p + d
            if not n.isInside():
                continue
            # print(f"n {n} {grid[n.x][n.y]}")
            if grid[n.x][n.y] !=  '#' and depths[n.x][n.y] > depths[p.x][p.y] + 1:
                depths[n.x][n.y] = depths[p.x][p.y] + 1
                if p.x != start.x or p.y != start.y:
                    nexts[start.x][start.y][n.x][n.y] = nexts[start.x][start.y][p.x][p.y]
                if grid[n.x][n.y] == ' ':
                    queue.append(n)
            if n.x == end.x and n.y == end.y:
                return depths[n.x][n.y]

start = Point(w//2, w//2)
grid[start.x][start.y] = '.'
location = start
station = Point(-1,-1)
while True:
    out = bfs(location)
    if out is None:
        for y in range(w):
            for x in range(w):
                if x == start.x and y == start.y:
                    print("X", end="")
                else:
                    print(grid[x][y], end="")
            print()
        print(f"station at {station}")
        print(dist(start, station))
        break
    # print(f"out {out}")
    amp.inputs.append(out)
    res = amp.run()
    # print(f"res {res}")
    neigh = location + dirs[out - 1]
    # print(f"considering neigh {neigh}")
    print(f"from {location} to {neigh} (dir {out}) = {res}")
    if res == 0:
        grid[neigh.x][neigh.y] = '#'
    else:
        grid[neigh.x][neigh.y] = ' '
        location = neigh

    if res == 2:
        station = neigh
        print(f"station at {neigh}")
        # exit()
    #time.sleep(1)


def disperse():
    for x in range(w):
        for y in range(w):
            if grid[x][y] == 'O':
                for d in dirs:
                    n = Point(x, y) + d
                    if n.isInside() and grid[n.x][n.y] == ' ':
                        grid[n.x][n.y] = 'o'
    found = False
    for x in range(w):
        for y in range(w):
            if grid[x][y] == 'o':
                grid[x][y] = 'O'
                found = True
    return found

timer = 0
grid[station.x][station.y] = 'O'
while disperse():
    timer += 1
print(f"total timer {timer}")