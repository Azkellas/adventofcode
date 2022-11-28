line = input()
intsSave = [int(i) for i in line.split(',')]

def tryNumbers(a, b):
    ints = intsSave.copy()
    ints[1] = a
    ints[2] = b
    idx = 0
    while True:
        op = ints[idx]
        if not op in [1, 2, 99]:
            print('weird op ', op)
        if op == 99:
            if ints[0] == 19690720:
                print('found: a = ', a, ', b = ', b)
                print(100*a+b)
            return

        id1, id2, tar = ints[idx+1], ints[idx+2], ints[idx+3]
        val1, val2 = ints[id1], ints[id2]
        res = val1 + val2 if op == 1 else val1 * val2
        ints[tar] = res
        idx += 4    



for a in range(101):
    for b in range(101):
        tryNumbers(a, b)