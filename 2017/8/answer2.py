import sys
import re

registers = {"nokey": -1000000000}


maxi = -100000
for line in sys.stdin:
    line = line.strip()
    m = re.match("(\w+) (\w+) (\S+) if (\w+) (\S+) (\S+)", line)
    # print(line)
    m = m.groups()
    # print(m.groups())
    reg1, cmd, val, cnd_reg, cnd, cnd_val = m
    val = int(val)
    cnd_val = int(cnd_val)
    cr_val = registers[cnd_reg] if cnd_reg in registers else 0

    r_val = registers[reg1] if reg1 in registers else 0
    if eval(f"{cr_val} {cnd} {cnd_val}"):
        if   cmd == "inc": registers[reg1] = r_val + val
        elif cmd == "dec": registers[reg1] = r_val - val
        else: print(f"unknwon {cmd}")
    maxi = max(maxi, max(registers.values()))

print(maxi)