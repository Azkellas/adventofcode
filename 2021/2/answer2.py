import sys

lines = sys.stdin
hor = 0
ver = 0
aim = 0
for line in lines:
    command, val = line.split()
    val = int(val)
    if command == "forward":
        hor = hor + val
        ver = ver + aim * val
    if command == "down":    aim = aim + val
    if command == "up":      aim = aim - val

print(hor * ver)