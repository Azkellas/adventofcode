from sys import stdin

line = input()

x1 = y1 = 0
x2 = y2 = 0

houses = {(x1, y1): 1}

dx = {'^': 0, 'v': 0, '<': -1, '>': 1}
dy = {'^': -1, 'v': 1, '<': 0, '>': 0}

for i in range(len(line)):
    c = line[i]
    if i % 2 == 0:
        x1 += dx[c]
        y1 += dy[c]
    else:
        x2 += dx[c]
        y2 += dy[c]

    houses[(x1, y1)] = 1
    houses[(x2, y2)] = 1

print(houses.keys())
print(len(houses.keys()))
