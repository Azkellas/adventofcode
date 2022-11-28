val = int(input())
target = val
val -= 1

dirs = ['R', 'U', 'L', 'D']

baseX = 500
baseY = 500

offX = 0
offY = 0
l = 0
dir = 'R'

grid = [ [0 for i in range(1000)] for j in range(1000)]
grid[baseX][baseY] = 1
while val > 0:
    if dir == 'R' or dir == 'L':
        l += 1

    ll = min(l, val)
    val -= ll
    for k in range(ll):
        if dir == 'R':
            offX += 1
        if dir == 'L':
            offX -= 1
        if dir == 'U':
            offY -= 1
        if dir == 'D':
            offY += 1

        pX = baseX + offX
        pY = baseY + offY
        score = 0
        score += grid[pX-1][pY]
        score += grid[pX-1][pY-1]
        score += grid[pX-1][pY+1]
        score += grid[pX][pY-1]
        score += grid[pX][pY+1]
        score += grid[pX+1][pY]
        score += grid[pX+1][pY-1]
        score += grid[pX+1][pY+1]
        grid[pX][pY] = score
        if score > target:
            print("end = ", score)
            exit()

    dir = dirs[(dirs.index(dir) + 1) % len(dirs)]
print(offX, offY)
print(abs(offX) + abs(offY))

