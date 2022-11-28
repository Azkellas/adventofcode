import sys
import re

mem = {}

for line in sys.stdin:
    match_mask = re.match(r'mask = (\w+)', line)
    if match_mask:
        mask = match_mask.group(1)
        print(mask)
    mem_mask = re.match(r'mem\[(\d+)\] = (\d+)', line)
    if mem_mask:
        a = int(mem_mask.group(1))
        v = int(mem_mask.group(2))
        print('mem', a, v)
        for i in range(1, 37):
            if mask[-i] == '0':
                v &= ~(1 << (i - 1))
            if mask[-i] == '1':
                v |= 1 << (i - 1)
        mem[a] = v

print(sum(mem.values()))
