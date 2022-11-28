banks = [int(i) for i in input().split()]
print(banks)

seens = set()

step = 0
step1 = 0
while True:
    s = ",".join([str(i) for i in banks])
    print(s)
    if s in seens:
        print(step - step1)
        if step1 == 0:
            step1 = step
            seens.clear()
        else:
            exit()
    seens.add(s)
    step = step + 1

    m,M = -1, -1
    for i,v in enumerate(banks):
        if v > M: m,M = i,v

    banks[m] = 0
    while M > 0:
        m = m + 1
        if m >= len(banks): m = 0
        banks[m] = banks[m] + 1
        M = M - 1
    
print(step)