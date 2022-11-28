import copy 

nbs = [int(n) for n in input().split(',')]
print(nbs)


days   = [0 for i in range(7)]
babies = [0 for i in range(7)]
for n in nbs:
    days[n] = days[n] + 1

for d in range(256):
    m = d % 7
    # t = days[m]
    babies[(m + 2) % 7] = days[m]
    days[m] = days[m] + babies[m]
    babies[m] = 0

print(sum(days) + sum(babies))
