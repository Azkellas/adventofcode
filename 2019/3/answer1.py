import math
from sys import stdin

width = 17000
startX = 8800
startY = 6350
mat = []
for i in range(width):
    mat.append([0]*width)

distMin = width*width
lId = 0
for line in stdin:
    lId += 1
    print("beg")
    x = startX
    y = startY
    mat[x][y] = 0
    path = line.split(',')
    for command in path:
        dir = command[0]
        count = int(command[1:])
        for i in range(count):
            if dir == 'U':
                y -= 1
            if dir == 'R':
                x += 1
            if dir == 'L':
                x -= 1
            if dir == 'D':
                y += 1
            if mat[x][y] == 1 and lId == 2 and (x != startX or y != startY):
                print("new ", x, y)
                distMin = min(distMin, abs(y-startY) + abs(x - startX))
            mat[x][y] = lId
        if x < 0 or x > width-1 or y < 0 or y > width-1:
            print("oob", x, y)


print(distMin)
print()
# print(mat)