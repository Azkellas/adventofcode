target_y, target_x = 2978 - 1, 3083 - 1

height, width = 3084, 3084

debug = [[0 for i in range(6)] for j in range(6)]
x, y = 0, 0
n = 20151125
while x != target_x or y != target_y:
    if x < 6 and y < 6:
        debug[x][y] = n

    # grid[x][y] = n
    n = (n * 252533) % 33554393

    if y > 0:
        y -= 1
        x += 1
    else:
        y = x + 1
        x = 0

print(n)
