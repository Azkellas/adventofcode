import sys
import re



lines = []
for line in sys.stdin:
    line = line.strip()
    [cmd, val] = line.split()
    val = int(val)
    print(cmd, val)
    lines.append((cmd, val))

acc = 0
current_line = 0
seen = set()

while not current_line in seen:
    seen.add(current_line)
    (cmd, val) = lines[current_line]
    if cmd == 'acc':
        acc += val
    
    if cmd == 'jmp':
        current_line += val
    else:
        current_line += 1

print(acc)