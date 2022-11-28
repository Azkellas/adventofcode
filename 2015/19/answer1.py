import sys

rules = []

is_react = True
for line in sys.stdin:
    line = line.strip()
    if line == '':
        is_react = False
    
    elif is_react:
        line = line.split('=>')
        r_from = line[0].strip()
        r_to   = line[1].strip()
        rules.append((r_from, r_to))
        
    else:
        formula = line    


length = len(formula)

valid = set()

for i in range(length):
    for (r_from, r_to) in rules:
        if formula[i:].startswith(r_from):
            valid.add(formula[:i] + formula[i:].replace(r_from, r_to, 1))

print(len(valid))
