import sys

lines = sys.stdin
hor = 0
ver = 0
for line in lines:
    command, val = line.split()
    val = int(val)
    if command == "forward": hor = hor + val
    if command == "down":    ver = ver + val
    if command == "up":      ver = ver - val

print(hor * ver)