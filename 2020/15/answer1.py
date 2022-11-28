mem = [int(i) for i in input().split(',')]
mem.reverse()

l_input = len(mem)
for i in range(l_input, 2020):
    v = mem[0]
    try:
        idx = mem.index(v, 1)
        mem.insert(0, idx)
    except:
        mem.insert(0, 0)

print(mem[0])