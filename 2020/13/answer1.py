m = int(input())
l_buses = input().split(',')
buses = []
for b in l_buses:
    if b != 'x':
        buses.append(int(b))


res = 100000
bid = -1
for b in buses:
    if m % b == 0:
        res = 0
    else:
        s = b * (1 + (m // b)) - m
        if s < res:
            res = s
            bid = b
print(res * bid)