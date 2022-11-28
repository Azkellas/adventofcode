import sys
import math 

cmds = []
for line in sys.stdin:
    line = line.split()
    if line[0] == 'inp': line.append('0')
    cmd, a, b = line
    # if b not in ['x', 'y', 'z', 'w']:
    #     b = int(b)
    cmds.append((cmd, a, b))

regs = {'x': '0', 'y': '0', 'z': '0', 'w': '0'}

def get_val(a):
    if a in 'xyzw': return regs[a]
    return a
    

n = [1 for i in range(14)]
n = "abcdefghijklmn"

conditions = []

# n = [9 for i in range(14)]
while True:
    # print(''.join([str(i) for i in n]))
    regs = {'x': '0', 'y': '0', 'z': '0', 'w': '0'}
    input_i = 0
    for cmd, a, b in cmds:
        if cmd == 'inp':
            regs[a] = n[input_i]
            input_i += 1
        if cmd == 'add':
            if get_val(b) != '0' and get_val(b) != '26':
                if regs[a] == '0' or regs[a] == '26':
                    regs[a] = get_val(b)
                else:
                    regs[a] = f"({regs[a]} + {get_val(b)})"

        if cmd == 'mul':
            if get_val(b) == '0' or get_val(b) == '26':
                regs[a] = '0'
            elif regs[a] == '0' or regs[a] == '26':
                regs[a] = '0'
            elif get_val(b) == '1':
                regs[a] = regs[a]
            elif regs[a] == '1':
                regs[a] = get_val(b)
            else:
                regs[a] = f"({regs[a]} * {get_val(b)})"

        if cmd == 'div':
            if get_val(b) != '1':
                regs[a] = f"({regs[a]} / {get_val(b)})"

        if cmd == 'mod':
            if get_val(b) != '26':
                regs[a] = f"({regs[a]} % {get_val(b)})"
        if cmd == 'eql': regs[a] = f"({regs[a]} == {get_val(b)})"

        if cmd == 'eql':
            new = True
            if regs[a] == "(14 == a)":
                new = False
                regs[a] = "0"
                regs['z'] = "0"
            if regs[a] == "(0 == 0)":
                new = False
                regs[a] = "1"

            if regs[a] == "(((a + 8) + 15) == b)":
                new = False
                regs[a] = "(a - 3 == b)"
            if regs[a] == "((a - 3 == b) == 0)":
                new = False
                regs[a] = "C1"
                conditions.append('a - 3 != b')
                print(regs)
                exit()

            if regs[a] == "(((((a + 8) * ((25 * C1) + 1)) + ((b + 11) * C1)) + 13) == c)":
                new = False
                regs['x'] = '([a - 5, b - 2][C1] == c)'
                regs['z'] = '[a + 8, b + 11][C1]'
            if regs[a] == "(([a - 5, b - 2][C1] == c) == 0)":
                new = False
                regs[a] = "C2"
                conditions.append('[a - 5, b - 2][C1] != c')

            if regs[a] == '(((([a + 8, b + 11][C1] * ((25 * C2) + 1)) + ((c + 2) * C2)) + -10) == d)':
                new = False
                regs['x'] = '([[a - 2, b + 1][C1], c - 8][C2] == d)'
                regs['y'] = '((c + 2) * C2)'
                regs['z'] = '[a + 8, b + 11][C1]'
            if regs[a] == "(([[a - 2, b + 1][C1], c - 8][C2] == d) == 0)":
                new = False
                regs[a] = "C3"
                conditions.append('[[a - 2, b + 1][C1], c - 8][C2] != d)')

            if regs[a] == '(((([a + 8, b + 11][C1] * ((25 * C3) + 1)) + ((d + 11) * C3)) + 14) == e)':
                new = False
                regs['x'] = '([[a - 4, b - 1][C1], d - 1][C3] + 14 == e)'
                regs['y'] = '((c + 2) * C2)'
                regs['z'] = '[[a + 8, b + 11][C1], d + 11][C3]'
            if regs[a] == "(([[a - 4, b - 1][C1], d - 1][C3] + 14 == e) == 0)":
                new = False
                regs[a] = "C4"
                conditions.append('[[a - 4, b - 1][C1], d - 1][C3] + 14 != e')

            if regs[a] == '(((([[a + 8, b + 11][C1], d + 11][C3] * ((25 * C4) + 1)) + ((e + 1) * C4)) + -3) == f)':
                new = False
                regs['x'] = '([[[a + 5, b + 8][C1], d + 8][C3], e - 2][C4] == f)'
                regs['y'] = '((c + 2) * C2)'
                regs['z'] = '([[a + 8, b + 11][C1], d + 11][C3] * C4)'
            if regs[a] == "(([[[a + 5, b + 8][C1], d + 8][C3], e - 2][C4] == f) == 0)":
                new = False
                regs[a] = "C5"
                conditions.append('[[[a + 5, b + 8][C1], d + 8][C3], e - 2][C4] != f')


            print()
            print(a, ':',regs[a])
            print(regs)
            if new:
                exit()
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