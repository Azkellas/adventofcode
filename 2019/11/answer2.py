line = input()
codes = [int(i) for i in line.split(',')]
for i in range(1000):
    codes.append(0)

idx = 0
inputs = []
relativeBase = 0

def getVal(codes, idx, t, base, target=False):
    if t == 0:
        if target:
            return idx
        else:
            return codes[idx]
    elif t == 1:
        return idx
    elif t == 2:
        if target:
            return base + idx
        else:
            return codes[base + idx]
    else:
        print("issue in getVal", idx, t, base)

def Amp():
    global idx, relativeBase
    while True:
        opcode = str(codes[idx])
        op = int(opcode[-2:])
        t1 = int(opcode[-3]) if len(opcode) >= 3 else 0
        t2 = int(opcode[-4]) if len(opcode) >= 4 else 0
        tTar = int(opcode[-5]) if len(opcode) >= 5 else 0

        if not op in [1, 2, 3, 4, 5, 6, 7, 8, 9, 99]:
            print('weird op ', op)
        if op == 99:
            print('end', i)
            return -1

        # multiple vals
        if op in [1, 2]:
            # print(codes[idx:idx+4])
            id1, id2, idTar = codes[idx+1], codes[idx+2], codes[idx+3]
            val1 = getVal(codes, id1, t1, relativeBase)
            val2 = getVal(codes, id2, t2, relativeBase)
            tar = getVal(codes, idTar, tTar, relativeBase, True)
            
            res = val1 + val2 if op == 1 else val1 * val2
            codes[tar] = res
            idx += 4

        if op == 3:
            if len(inputs) == 0:
                print("no input!")
                return
            inputVal = inputs.pop(0)
            id1 = codes[idx+1]
            val1 = getVal(codes, id1, t1, relativeBase, True)
            codes[val1] = inputVal
            idx += 2

        if op == 4:
            id1 = codes[idx+1]
            val1 = getVal(codes, id1, t1, relativeBase)

            # print("output: ", val1)
            idx += 2
            return val1

        if op in [5, 6]:
            # print(codes[idx:idx+3])
            id1, id2 = codes[idx+1], codes[idx+2]
            val1 = getVal(codes, id1, t1, relativeBase)
            val2 = getVal(codes, id2, t2, relativeBase)
            if op == 5 and val1 or op == 6 and not val1:
                idx = val2
            else:
                idx += 3

        if op in [7, 8]:
            # print(codes[idx:idx+4])
            id1, id2, idTar = codes[idx+1], codes[idx+2], codes[idx+3]
            val1 = getVal(codes, id1, t1, relativeBase)
            val2 = getVal(codes, id2, t2, relativeBase)
            tar = getVal(codes, idTar, tTar, relativeBase, True)
            codes[tar] = int(op == 7 and val1 < val2 or op == 8 and val1 == val2)
            idx += 4

        if op == 9:
            # print(codes[idx:idx+2])
            id1 = codes[idx+1]
            val1 = getVal(codes, id1, t1, relativeBase)
            relativeBase += val1
            idx += 2

w = 2000
colors = [ [0 for i in range(w)] for j in range(w)]
painted = [ [False for i in range(w)] for j in range(w)]
base = w//2
x, y = base, base
colors[x][y] = 1

dir = 'U'
dirs = ['U', 'R', 'D', 'L']
while dir != 'X':
    inputs.append(colors[x][y])
    color = Amp()
    if color == -1:
        break
    turn = Amp()
    print(color, turn)
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

minX = w
maxX = 0
minY = w
maxY = 0

res = 0
for x in range(w):
    for y in range(w):
        res += painted[x][y]
        if colors[x][y] == 1:
            minX = min(minX, x)
            maxX = max(maxX, x)
            minY = min(minY, y)
            maxY = max(maxY, y)

print(f"between ({minX}, {minY}) and ({maxX}, {maxY})")
for y in range(minY, maxY+1):
    for x in range(minX, maxX+1):
        print([' ', '#'][colors[x][y]], end="")
    print()


print("1:", res)