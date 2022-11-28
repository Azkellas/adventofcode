import sys

wx, wy = 10, -1
bx, by = 0, 0

# dir = 'E'
# dirs = ['N', 'E', 'S', 'W']
# dlts = {
#     'N': ( 0, -1),
#     'E': ( 1,  0),
#     'S': ( 0,  1),
#     'W': (-1,  0),
#     }

for line in sys.stdin:
    c = line[0]
    v = int(line[1:])

    if c == 'E': wx += v
    if c == 'W': wx -= v
    if c == 'N': wy -= v
    if c == 'S': wy += v

    if c == 'F':
        bx += v * wx
        by += v * wy
    
    if c == 'L' or c == 'R':
        print(v)
        if v == 180:
            wx, wy = -wx, -wy
        else:
            if v != 90 and v != -90:
                if v == 270:
                    v = -90
                if v == -270:
                    v = 90
                # print('=================================')
                # print(v)
            # 90+
            wx, wy = -wy, wx
            if c == 'L' and v == 90 or c == 'R' and v == -90:
                wx, wy = -wx, -wy

    # print(f'({bx}, {by}), ({wx}, {wy})')

print(abs(bx) + abs(by))

