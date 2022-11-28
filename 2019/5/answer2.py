line = input()
codes = [int(i) for i in line.split(',')]
print(codes)

inputVal = 5

# codes[1] = 12
# codes[2] = 2

idx = 0

while True:
    opcode = str(codes[idx])
    op = int(opcode[-2:])
    t1 = int(opcode[-3]) if len(opcode) >= 3 else 0
    t2 = int(opcode[-4]) if len(opcode) >= 4 else 0
    tTar = int(opcode[-5]) if len(opcode) >= 5 else 0
    print(op, t1,t2,tTar)

    if not op in [1, 2, 3, 4, 5, 6, 7, 8, 99]:
        print('weird op ', op)
    if op == 99:
        print('end')
        print(codes[0])
        break

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
        tar = codes[idx+1]
        codes[tar] = inputVal
        idx += 2

    if op == 4:
        id1 = codes[idx+1]
        res = id1 if t1 else codes[id1]
        print("output: ", res)
        idx += 2

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

        codes[tar] = op == 7 and val1 < val2 or op == 8 and val1 == val2
        idx += 4