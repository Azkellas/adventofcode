import sys
from queue import Queue

grid = []

for line in sys.stdin:
    line = line.strip()
    grid.append([int(c) for c in line])

lx = len(grid[0])
ly = len(grid)

sizes = []
clic_id = 0

for x in range(lx):
    for y in range(ly):
        if 0 <= grid[y][x] < 9:
            clic_id = clic_id - 1
            clic_size = 0
            q = Queue()
            q.put((y, x))
            while not q.empty():
                a, b = q.get()
                if (grid[a][b] < 0):
                    continue

                grid[a][b] = clic_id
                clic_size = clic_size + 1
                if a > 0    and 0 <= grid[a-1][b] < 9: q.put((a-1, b))
                if a < ly-1 and 0 <= grid[a+1][b] < 9: q.put((a+1, b))
                if b > 0    and 0 <= grid[a][b-1] < 9: q.put((a, b-1))
                if b < lx-1 and 0 <= grid[a][b+1] < 9: q.put((a, b+1))
            sizes.append(clic_size)
print(sizes)
sizes.sort(reverse=True)
sizes = sizes[:3]
print(sizes)
score = 1
for s in sizes:
    score = score * s
print(score)

# for y in range(ly):
#     for x in range(lx):
#         print(grid[y][x], end=" ")
#     print()
