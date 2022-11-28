import sys
import math 

cmds = []
for line in sys.stdin:
    line = line.split()
    if line[0] == 'inp': line.append('0')
    cmd, a, b = line
    if b not in ['x', 'y', 'z', 'w']:
        b = int(b)
    cmds.append((cmd, a, b))

regs = {'x': 0, 'y': 0, 'z': 0, 'w': 0}

def get_val(a):
    if isinstance(a, int): return a
    return regs[a]

n = [1 for i in range(14)]
# n = [9 for i in range(14)]
while True:
    # print(''.join([str(i) for i in n]))
    regs = {'x': 0, 'y': 0, 'z': 0, 'w': 0}
    input_i = 0
    for cmd, a, b in cmds:
        if cmd == 'inp':
            regs[a] = n[input_i]
            input_i += 1
        if cmd == 'add': regs[a] = regs[a] + get_val(b)
        if cmd == 'mul': regs[a] = regs[a] * get_val(b)
        if cmd == 'div': regs[a] = math.trunc(regs[a] / get_val(b))
        if cmd == 'mod': regs[a] = regs[a] % get_val(b)
        if cmd == 'eql': regs[a] = [0, 1][regs[a] == get_val(b)]
    if regs['z'] == 0:
        # print(n)
        print(''.join([str(i) for i in n]))
        print(regs, input_i)
        # print(n[0] - n[1])
        exit(1)
    
    # i = 13
    # while True:
    #     if n[i] > 1:
    #         n[i] = n[i] - 1
    #         break
    #     else:
    #         n[i] = 9
    #         i = i - 1
    i = 0
    while True:
        if n[i] < 9:
            n[i] = n[i] + 1
            break
        else:
            n[i] = 1
            i = i + 1