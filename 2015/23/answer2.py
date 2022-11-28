import sys


inst = []

for line in sys.stdin:
    line = line.replace(',', '').strip().split()
    inst.append(line)

rgs = {'a': 1, 'b': 0}

idx = 0

while 0 <= idx < len(inst):
    line = inst[idx]
    if   line[0] == 'hlf':
        rgs[line[1]] /= 2
        idx += 1
    elif line[0] == 'tpl':
        rgs[line[1]] *= 3
        idx += 1
    elif line[0] == 'inc':
        rgs[line[1]] += 1
        idx += 1
    elif line[0] == 'jmp':
        idx += int(line[1])
    elif line[0] == 'jie':
        if rgs[line[1]] % 2 == 0:
            idx += int(line[2])
        else:
            idx += 1
    elif line[0] == 'jio':
        if rgs[line[1]] == 1:
            idx += int(line[2])
        else:
            idx += 1
    else:
        print('error')

print(rgs['b'])
