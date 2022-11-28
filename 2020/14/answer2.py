import sys
import re
import itertools

mem = {}

for line in sys.stdin:
    match_mask = re.match(r'mask = (\w+)', line)
    if match_mask:
        mask = match_mask.group(1)
        # print(mask)

    mem_mask = re.match(r'mem\[(\d+)\] = (\d+)', line)
    if mem_mask:
        a = int(mem_mask.group(1))
        v = int(mem_mask.group(2))
        # print('mem', a, v)
        undefs = []
        for i in range(1, 37):
            if mask[-i] == '1':
                a |= 1 << (i - 1)
            if mask[-i] == 'X':
                undefs.append(i-1)
        
        for L in range(0, len(undefs)+1):
            for subset in itertools.combinations(undefs, L):
                adr = a
                for i in undefs:
                    if i in subset:
                        adr |= 1 << i
                    else:
                        adr &= ~(1 << i)
                mem[adr] = v

print(sum(mem.values()))
print(len(mem))