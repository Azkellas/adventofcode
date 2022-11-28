import sys

def eval(l):
    tot = 0
    idx = 0
    op = '+'
    for c in l:
        try:
            a = int(c)
            tot = [tot * a, tot + a][op == '+']
        except:
            op = c
    return tot


total = 0
for line in sys.stdin:
    line = line.strip()
    line = line.replace('(', '( ')
    line = line.replace(')', ' )')
    line = line.split(' ')
    while '(' in line:
        # print(' '.join(line))

        idx1, idx2 = -1, -1
        for i in range(len(line)):
            if idx1 != -1:
                break
            if line[i] != '(':
                continue
            for j in range(i + 1, len(line)):
                if line[j] == '(':
                    break
                if line[j] == ')':
                    idx1, idx2 = i, j
                    break

        t = eval(line[idx1+1:idx2])
        # print(idx1, idx2, t)
        line = line[:idx1] + [str(t)] + line[idx2+1:]
    print('== ', eval(line))
    total += eval(line)

print(total)