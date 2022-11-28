from sys import stdin

line = input()

x = 0
y = 0
houses = {(x, y): 1}

dx = {'^': 0, 'v': 0, '<': -1, '>': 1}
dy = {'^': -1, 'v': 1, '<': 0, '>': 0}
for c in line:
    x += dx[c]
    y += dy[c]
    houses[(x, y)] = 1

print(houses.keys())
print(len(houses.keys()))
