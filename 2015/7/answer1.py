from sys import stdin
import re
def parse(line):
    if re.compile("NOT (.*) -> (.*)").findall(line):
        return ["NOT"] + re.compile("NOT (.*) -> (.*)").findall(line)
    elif re.compile("(.*) OR (.*) -> (.*)").findall(line):
        return ["OR"] + re.compile("(.*) OR (.*) -> (.*)").findall(line)
    elif re.compile("(.*) AND (.*) -> (.*)").findall(line):
        return ["AND"] + re.compile("(.*) AND (.*) -> (.*)").findall(line)
    elif re.compile("(.*) RSHIFT (.*) -> (.*)").findall(line):
        return ["RSHIFT"] + re.compile("(.*) RSHIFT (.*) -> (.*)").findall(line)
    elif re.compile("(.*) LSHIFT (.*) -> (.*)").findall(line):
        return ["LSHIFT"] + re.compile("(.*) LSHIFT (.*) -> (.*)").findall(line)
    elif re.compile("(.*) -> (.*)").findall(line):
        return ["VAL"] + re.compile("(.*) -> (.*)").findall(line)
    else:
        print("UNKNOWN", line)

wires = {}
existing = {}

ops = []
for line in stdin:
    res = parse(line)
    op = res[0]
    for n in res[1]:
        if not n.isdigit():
            existing[n] = True

    ops.append(res)

def ok(n):
    return n.isdigit() or n in wires

def val(n):
    if n.isdigit():
        return int(n)
    else:
        return wires[n]

while len(wires) != len(existing):
    print(len(existing) - len(wires))
    for op in ops:

        opera = op[0]
        numb = op[1]

        if numb[-1] in wires:
            continue

        if opera == "VAL":
            if ok(numb[0]):
                wires[numb[1]] = val(numb[0])

        elif opera == "NOT" and ok(numb[0]):
            wires[numb[1]] = ~val(numb[0])
        
        elif opera == "OR" and ok(numb[0]) and ok(numb[1]):
            wires[numb[2]] = val(numb[0]) | val(numb[1])

        elif opera == "AND" and ok(numb[0]) and ok(numb[1]):
            wires[numb[2]] = val(numb[0]) & val(numb[1])

        elif opera == "LSHIFT" and ok(numb[0]):
            wires[numb[2]] = val(numb[0]) << val(numb[1])

        elif opera == "RSHIFT" and ok(numb[0]):
            wires[numb[2]] = val(numb[0]) >> val(numb[1])
    print('m')

print(wires)
print(sorted(existing.keys()))

print(wires["a"])
