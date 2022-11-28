import sys
import re



lines = []
for line in sys.stdin:
    line = line.strip()
    [cmd, val] = line.split()
    val = int(val)
    lines.append((cmd, val))

line_count = len(lines)
for i in range(line_count):
    if lines[i][0] == 'acc':
        continue
    
    acc = 0
    current_line = 0
    seen = set()

    while not current_line in seen and current_line < line_count:
        seen.add(current_line)
        (cmd, val) = lines[current_line]
        if current_line == i:
            cmd = ['nop', 'jmp'][cmd == 'nop']

        if cmd == 'acc':
            acc += val
        
        if cmd == 'jmp':
            current_line += val
        else:
            current_line += 1

    if current_line == line_count:
        print(acc)
        break