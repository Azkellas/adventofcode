from functools import reduce

m = int(input())
l_buses = input().split(',')
buses = []
for b in l_buses:
    if b != 'x':
        buses.append(int(b))

# Python 3.6
def chinese_remainder(n, a):
    sum = 0
    prod = reduce(lambda a, b: a*b, n)
    for n_i, a_i in zip(n, a):
        p = prod // n_i
        sum += a_i * mul_inv(p, n_i) * p
    return sum % prod
 
 
 
def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1: return 1
    while a > 1:
        q = a // b
        a, b = b, a%b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0: x1 += b0
    return x1
 


nn = []
aa = []

for i in range(len(l_buses)):
    b = l_buses[i]
    if b != 'x':
        nn.append(int(b))
        # aa.append(i % int(b))
        aa.append(-i)


print(nn, aa)
res = chinese_remainder(nn, aa)
print(res)
for b in buses:
    print(b, res % b)

res = 1068781
for b in buses:
    print(b, res % b)
