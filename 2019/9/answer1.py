line = input()
codes = [int(i) for i in line.split(',')]
print(codes)
for i in range(1000):
    codes.append(0)

inputVal = 1

# codes[1] = 12
# codes[2] = 2


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

idx = 0
relativeBase = 0
while True:
    opcode = str(codes[idx])
    op = int(opcode[-2:])
    t1 = int(opcode[-3]) if len(opcode) >= 3 else 0
    t2 = int(opcode[-4]) if len(opcode) >= 4 else 0
    tTar = int(opcode[-5]) if len(opcode) >= 5 else 0

    if not op in [1, 2, 3, 4, 5, 6, 7, 8, 9, 99]:
        print('weird op ', op)
        break
    if op == 99:
        print('end')
        print(codes[0])
        break

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
        # print(codes[idx:idx+2])
        id1 = codes[idx+1]
        val1 = getVal(codes, id1, t1, relativeBase, True)

        # print("save", val1, id1, t1, relativeBase, inputVal)
        # print(codes[idx:idx+2], relativeBase + id1, codes[relativeBase+id1])
        codes[val1] = inputVal
        idx += 2

    if op == 4:
        # print(codes[idx:idx+2])
        id1 = codes[idx+1]
        val1 = getVal(codes, id1, t1, relativeBase)

        print("output: ", val1)
        idx += 2

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