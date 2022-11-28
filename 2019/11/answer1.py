line = input()
codes = [int(i) for i in line.split(',')]
for i in range(1000):
    codes.append(0)

idx = 0
inputs = []

def Amp():
    global idx
    print(idx)
    while True:
        opcode = str(codes[idx])
        op = int(opcode[-2:])
        t1 = int(opcode[-3]) if len(opcode) >= 3 else 0
        t2 = int(opcode[-4]) if len(opcode) >= 4 else 0
        tTar = int(opcode[-5]) if len(opcode) >= 5 else 0

        if not op in [1, 2, 3, 4, 5, 6, 7, 8, 99]:
            print('weird op ', op)
        if op == 99:
            print('end', i)
            return -1

        # multiple vals
        if op in [1, 2]:
            id1, id2, tar = codes[idx+1], codes[idx+2], codes[idx+3]
            # print(op, id1, id2, tar)
            # print()
            val1 = id1 if t1 else codes[id1]
            val2 = id2 if t2 else codes[id2]
            res = val1 + val2 if op == 1 else val1 * val2
            codes[tar] = res
            idx += 4

        if op == 3:
            if len(inputs) == 0:
                print("waiting")
                return  # waiting for next input
            tar = codes[idx+1]
            print(op, tar, idx)
            val = inputs.pop(0)
            codes[tar] = val
            idx += 2

        if op == 4:
            id1 = codes[idx+1]
            res = id1 if t1 else codes[id1]
            print("output: ", res)
            idx += 2
            return res

        if op in [5, 6]:
            id1, id2 = codes[idx+1], codes[idx+2]
            val1 = id1 if t1 else codes[id1]
            val2 = id2 if t2 else codes[id2]
            if op == 5 and val1 or op == 6 and not val1:
                idx = val2
            else:
                idx += 3

        if op in [7, 8]:
            id1, id2, tar = codes[idx+1], codes[idx+2], codes[idx+3]
            val1 = id1 if t1 else codes[id1]
            val2 = id2 if t2 else codes[id2]

            codes[tar] = int(op == 7 and val1 < val2 or op == 8 and val1 == val2)
            idx += 4


w = 2000
colors = [ [0 for i in range(w)] for j in range(w)]
painted = [ [False for i in range(w)] for j in range(w)]
base = w//2
x, y = base, base

dir = 'U'
dirs = ['U', 'R', 'D', 'L']
while dir != 'X':
    inputs.append(colors[x][y])
    color = Amp()
    if color == -1:
        break
    turn = Amp()
    painted[x][y] = True
    colors[x][y] = color
    if turn == 0:
        dir = dirs[ (dirs.index(dir) - 1) % len(dirs)]
    elif turn == 1:
        dir = dirs[ (dirs.index(dir) + 1) % len(dirs)]

    if dir == 'U':
        y -= 1
    if dir == 'D':
        y += 1
    if dir == 'L':
        x -= 1
    if dir == 'R':
        x += 1

res = 0
for i in range(w):
    for j in range(w):
        res += painted[i][j]

print(res)