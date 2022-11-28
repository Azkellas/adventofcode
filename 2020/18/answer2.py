import sys

def eval(l):
    while '+' in l:
        idx = l.index('+')
        l = l[:idx-1] + [str(int(l[idx-1]) + int(l[idx+1]))] + l[idx+2:]

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
    print(line)
    line = line.replace('(', '( ')
    line = line.replace(')', ' )')
    line = line.split(' ')
    while '(' in line:
        print(' '.join(line))

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
    
    while '+' in line:
        print(' '.join(line))
        idx = line.index('+')
        line = line[:idx-1] + [str(int(line[idx-1]) + int(line[idx+1]))] + line[idx+2:]
    print(' '.join(line))
    print('== ', eval(line))
    total += eval(line)

print(total)