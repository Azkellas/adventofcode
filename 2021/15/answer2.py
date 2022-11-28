import sys
import copy
import os
import time

from queue import PriorityQueue

grid = []
for line in sys.stdin:
    grid.append([int(i) for i in line.strip()])

l = len(grid)

for i in range(1, 5):
    for y in range(l):
        for x in range(l):
            n = grid[y][x] + i
            if n >= 10: n = n - 9
            grid[y].append(n)

for i in range(1, 5):
    for y in range(l):
        z = [s+i for s in grid[y]]
        for o in range(len(z)):
            if z[o] >= 10: z[o] = z[o] - 9
        grid.append(z)

l = len(grid)

for y in range(l):
    for x in range(l):
        grid[y][x] = 10 + grid[y][x]

# for y in range(l):
#     if y % 10 == 0: print()
#     for x in range(l):
#         if x % 10 == 0: print(" ", end="")
#         print(f"{grid[y][x]:1}", end="")
#     print()

costs = [[1_000_000 for x in range(l)] for y in range(l)]

costs[0][0] = 0

q = PriorityQueue()

q.put((2*l - 2, 0, 0))

seen = 0
endit=False
while not q.empty():
    if endit: 
        break
    seen = seen + 1
    c, y, x = q.get()
    neighs = []
    if y > 0   and costs[y][x] + grid[y-1][x] < costs[y-1][x]: neighs.append((y-1, x))
    if y < l-1 and costs[y][x] + grid[y+1][x] < costs[y+1][x]: neighs.append((y+1, x))
    if x > 0   and costs[y][x] + grid[y][x-1] < costs[y][x-1]: neighs.append((y, x-1))
    if x < l-1 and costs[y][x] + grid[y][x+1] < costs[y][x+1]: neighs.append((y, x+1))

    for a, b in neighs:
        c = costs[y][x] + grid[a][b]
        costs[a][b] = c
        if a == l-1 and b == l-1:
            endit=True
            break
        q.put((c + 10 * (2*l - a - b - 2), a, b))
        # q.put((c, a, b))

    # for yy in range(l):
    #     for xx in range(l):
    #         if costs[yy][xx] == 1_000_000:
    #             print(end=' ')
    #         elif yy==y and xx==x:
    #             print(end='#')
    #         else:
    #             print(end='.')
    #     print()
    # time.sleep(0.01)
    # os.system("clear")
print(costs[l-1][l-1])
print(seen)
# for y in range(l):
#     for x in range(l):
#         print(f"{costs[y][x]:3}", end=" ")
#     print()