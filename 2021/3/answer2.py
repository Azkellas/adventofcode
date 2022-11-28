import sys
import copy

syslines = sys.stdin

lines = []

for line in syslines:
    lines.append(line.strip())

l = len(lines[0])
ones = [0 for i in range(l)]
zeroes = [0 for i in range(l)]

for line in lines:
    for i in range(l):
        if line[i] == '1': ones[i] = ones[i] + 1
        else:              zeroes[i] = zeroes[i] + 1

gamma = ''
epsilon = ''
for i in range(l):
    if ones[i] > zeroes[i]:
        gamma = gamma + '1'
        epsilon = epsilon + '0'
    else:
        gamma = gamma + '0'
        epsilon = epsilon + '1'

print(gamma, epsilon)
print(int(gamma, 2), int(epsilon, 2))
print(int(gamma, 2) * int(epsilon, 2))

ll = copy.deepcopy(lines)
for i in range(l):
    s = 0
    x = 0
    for a in ll:
        s = s + (a[i] == '1')
        x = x + (a[i] == '0')
    ll = [line for line in ll if line[i] == ['0', '1'][s >= x]]
    print(i, l, len(ll))
    # print(ll[0])
ox = ll[0]
print(ox)

ll = copy.deepcopy(lines)
for i in range(l):
    if len(ll) == 1:
        break
    s = 0
    x = 0
    for a in ll:
        s = s + (a[i] == '1')
        x = x + (a[i] == '0')
    ll = [line for line in ll if line[i] == ['1', '0'][x <= s]]
    print(i, l, len(ll))
    # print(ll[0])
tr = ll[0]
print(tr)

print(ox, tr)
print(int(ox, 2) * int(tr, 2))
