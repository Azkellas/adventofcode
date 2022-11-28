import sys
sys.path.append("..")
from Amp import Amp
codes = input()
amp = Amp(codes)

import time

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
    res = chr(res)
    if res == '\n':
        if len(grid[-1]) == 0:
            grid = grid[:-1]
            break
        grid.append([])
    else:
        grid[-1].append(res)

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
dirsName = ['L', 'U', 'R', 'D']

dirsP = {
    'L': dirs[0],
    'U': dirs[1],
    'R': dirs[2],
    'D': dirs[3]
}

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



dir = 'U'
path = ['L']
dir = 'L'
for y in range(height):
    for x in range(width):
        if grid[y][x] == '^':
            pos = Point(x, y)
            grid[y][x] = '#'
while True:
    next = pos + dirsP[dir]
    if next.isInside() and grid[next.y][next.x] == '#':
        pos = next
        if isinstance(path[-1], int):
            path[-1] += 1
        else:
            path.append(1)
    else:
        rightDir = (dirsName.index(dir) + 1) % 4
        neigh = pos + dirs[rightDir]
        if neigh.isInside() and grid[neigh.y][neigh.x] == '#':
            path.append('R')
            dir = dirsName[rightDir]
        leftDir = (dirsName.index(dir) - 1 + 4) % 4
        neigh = pos + dirs[leftDir]
        if neigh.isInside() and grid[neigh.y][neigh.x] == '#':
            path.append('L')
            dir = dirsName[leftDir]
        if path[-1] != 'R' and path[-1] != 'L':
            break

print(", ".join([str(i) for i in path]))
A = ['L', 4, 'L', 4, 'L', 6, 'R', 10, 'L', 6]
B = ['L', 12, 'L', 6, 'R', 10, 'L', 6]
C = ['R', 8, 'R', 10, 'L', 6]
seq = []
while True:
    okA = True
    for i in range(len(A)):
        if i >= len(path):
            okA = False
            break
        if A[i] != path[i]:
            okA = False
    if okA:
        seq.append('A')
        path = path[len(A):]

    okB = True
    for i in range(len(B)):
        if i >= len(path):
            okB = False
            break
        if B[i] != path[i]:
            okB = False
    if okB:
        seq.append('B')
        path = path[len(B):]

    okC = True
    for i in range(len(C)):
        if i >= len(path):
            okC = False
            break
        if C[i] != path[i]:
            okC = False
    if okC:
        seq.append('C')
        path = path[len(C):]

    if not okA and not okB and not okC:
        break

print(seq)
print(", ".join([str(i) for i in path]))


seq = ",".join(seq)
print(seq)

A = ",".join([str(c) for c in A])
print(A)

B = ",".join([str(c) for c in B])
print(B)

C = ",".join([str(c) for c in C])
print(C)

amp = Amp(codes)
amp.codes[0] = 2

print("\n\nres")
res = seq + '\n' + A + '\n' + B + '\n' + C + '\n' + 'n\n'
for c in res:
    print(c,end="")

for c in res:
    print(ord(c), end=", ")
    if c == "\n":
        print()
    amp.inputs.append(ord(c))

x = 0
y = 0
res = 'y'
while res != None:
    res = amp.run()
    if res == None:
        break
    #print(res, end=",")
    if res < 500:
        res = chr(res)
    print(res, end="")
    # if res == '\n':
    #     print()
    # if res == '\n':
    #     y += 1
    #     x = 0

    #     if y == height:
    #         y = 0
    #         for Y in range(height):
    #             for X in range(width):
    #                 if grid[Y][X] == '.':
    #                     print(' ', end="")
    #                 else:
    #                     print(grid[Y][X], end="")
    #             print()
    #         time.sleep(1)

    # else:
    #     grid[y][x] = res
    #     x += 1


print(res)