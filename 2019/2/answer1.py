line = input()
ints = [int(i) for i in line.split(',')]
print(ints)
idx = 0

ints[1] = 12
ints[2] = 2
while True:
    print(ints)
    op = ints[idx]
    if not op in [1, 2, 99]:
        print('weird op ', op)
    if op == 99:
        print('end')
        print(ints[0])
        break

    id1, id2, tar = ints[idx+1], ints[idx+2], ints[idx+3]
    print(op, id1, id2, tar)
    print()
    val1, val2 = ints[id1], ints[id2]
    res = val1 + val2 if op == 1 else val1 * val2
    ints[tar] = res
    idx += 4