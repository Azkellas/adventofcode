val = int(input())
val -= 1
dirs = ['R', 'U', 'L', 'D']

offX = 0
offY = 0
l = 0
dir = 'R'
while val > 0:
    if dir == 'R' or dir == 'L':
        l += 1

    ll = min(l, val)
    val -= ll
    if dir == 'R':
        offX += ll
    if dir == 'L':
        offX -= ll
    if dir == 'U':
        offY -= ll
    if dir == 'D':
        offY += ll
    dir = dirs[(dirs.index(dir) + 1) % len(dirs)]
print(offX, offY)
print(abs(offX) + abs(offY))

