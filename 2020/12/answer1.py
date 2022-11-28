import sys


x, y = 0, 0
dir = 'E'
dirs = ['N', 'E', 'S', 'W']
dlts = {
    'N': ( 0, -1),
    'E': ( 1,  0),
    'S': ( 0,  1),
    'W': (-1,  0),
    }

for line in sys.stdin:
    c = line[0]
    v = int(line[1:])

    if c == 'E': x += v
    if c == 'W': x -= v
    if c == 'N': y -= v
    if c == 'S': y += v

    if c == 'F':
        (dx, dy) = dlts[dir]
        x += v * dx
        y += v * dy
    
    if c == 'L':
        dir = dirs[(dirs.index(dir) - v//90 + 4) % 4] 
    if c == 'R':
        dir = dirs[(dirs.index(dir) + v//90 + 4) % 4] 


print(x, y)
print(abs(x) + abs(y))