import sys

from queue import Queue

grid = []
for line in sys.stdin:
    grid.append([int(i) for i in line.strip()])


l = len(grid)

costs = [[1_000_000 for x in range(l)] for y in range(l)]

costs[0][0] = 0

q = Queue()

q.put((0, 0))

while not q.empty():
    y, x = q.get()
    neighs = []
    if y > 0   and costs[y][x] + grid[y-1][x] < costs[y-1][x]: neighs.append((y-1, x))
    if y < l-1 and costs[y][x] + grid[y+1][x] < costs[y+1][x]: neighs.append((y+1, x))
    if x > 0   and costs[y][x] + grid[y][x-1] < costs[y][x-1]: neighs.append((y, x-1))
    if x < l-1 and costs[y][x] + grid[y][x+1] < costs[y][x+1]: neighs.append((y, x+1))

    for a, b in neighs:
        c = costs[y][x] + grid[a][b]
        costs[a][b] = c
        q.put((a, b))
print(costs[l-1][l-1])

for y in range(l):
    for x in range(l):
        print(f"{costs[y][x]:3}", end=" ")
    print()