from sys import stdin
import math

grid = []

width = -1
height = 0

for line in stdin:
    height += 1
    line = line.strip()
    grid.append(line)
    width = len(line)

bestX = -1
bestY = -1
bestScore = -1
for x in range(width):
    for y in range(height):
        if grid[x][y] != '#':
            continue  # cannot put a station here
        
        total = 0
        for dx in range(-width, width):
            for dy in range(-height, height):
                if dx == 0 and dy == 0:
                    continue
                if math.gcd(dx, dy) != 1:
                    continue

                currX = x + dx
                currY = y + dy
                while currX >= 0 and currX < width and currY >= 0 and currY < height:
                    if grid[currX][currY] == '#':
                        total += 1
                        break
                    currX += dx
                    currY += dy
                if total > bestScore:
                    bestScore = total
                    bestX = x
                    bestY = y

print(bestY, bestX, bestScore)