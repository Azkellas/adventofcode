from sys import stdin
import re

ON = 0
OFF = 1
SWITCH = 2
def getAction(line):
    reg = re.compile('(.*) (\d+),(\d+) through (\d+),(\d+)')
    res = reg.findall(line)
    if len(res):
        res = res[0]
        return res

lights = []
for i in range(1000):
    lights.append([0]*1000)

for line in stdin:
    res = getAction(line)
    action = res[0]
    x1, y1, x2, y2 = [int(i) for i in res[1:]]
    print(action, x1, y1, x2, y2)
    x = x1
    while x <= x2:
        y = y1
        while y <= y2:
            if action == 'turn on':
                lights[x][y] += 1
            elif action == 'turn off':
                lights[x][y] -= 1
                if lights[x][y] < 0:
                    lights[x][y] = 0
            elif action == 'toggle':
                lights[x][y] += 2
            y += 1
        x += 1

score = 0
for i in range(1000):
    for j in range(1000):
        score += lights[i][j]
print(score)
