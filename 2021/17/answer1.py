import re

# target area: x=29..73, y=-248..-194
line = input()

reg = "target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)"

m = re.match(reg, line)
xmin, xmax, ymin, ymax = [int(i) for i in m.groups()]
targetx = (xmin, xmax)
targety = (ymin, ymax)

boundsx = [1000, -1]
boundsy = [1000, -1]

xs = []
ys = []

for x in range(xmax):
    if targetx[0] <= x * (x + 1) // 2 <= targetx[1]:
        xs.append(x)
        if x < boundsx[0]: boundsx[0] = x
        if x > boundsx[1]: boundsx[1] = x

print(xs)
print(boundsx[1] - boundsx[0] + 1, len(xs))

for y_test in range(1000):
    dy = y_test
    y = 0
    while True:
        y = y + dy
        dy = dy - 1
        if y < targety[0]:
            break
        if y <= targety[1]:
            ys.append(y_test)
            if y < boundsy[0]: boundsy[0] = y
            if y > boundsy[1]: boundsy[1] = y
            break
print(ys)
print(boundsy[1] - boundsy[0] + 1, len(ys))

highest = 0
for startx in xs:
    for starty in ys:
        x,y = 0, 0
        dx,dy = startx, starty
        h = 0
        while True:
            x = x + dx
            y = y + dy
            if dx > 0: dx = dx - 1
            dy = dy - 1
            if y > h: h = y
            if y < targety[0]: break
            if y <= targety[1]:
                if h > highest: highest = h
                break
print(highest)