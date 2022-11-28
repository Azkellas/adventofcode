import sys
import re

rules = {}

my_passport = []
passports = []

step = 0
for line in sys.stdin:
    line = line.strip()
    if line == '':
        continue
    if line == 'your ticket:':
        step = 1
    elif line == 'nearby tickets:':
        step = 2
    elif step == 0:
        m = re.match('(.*): (\d+)-(\d+) or (\d+)-(\d+)', line)
        print(m.group(1), m.group(2), m.group(3), m.group(4), m.group(5))
        rules[m.group(1)] = ( (int(m.group(2)), int(m.group(3))) , (int(m.group(4)), int(m.group(5))) )
    elif step == 1:
        my_passport = [int(i) for i in line.split(',')]
    elif step == 2:
        passports.append([int(i) for i in line.split(',')])


tot = 0
new_passports = []
for passport in passports:
    all_valid = True
    for val in passport:
        valid = False
        for rule in rules:
            for r in rules[rule]:
                if r[0] <= val <= r[1]:
                    valid = True
        if not valid:
            all_valid = False

    if all_valid:
        new_passports.append(passport)

passports = new_passports

rule_count = len(rules)
rules_names = [False for i in range(rule_count)]

found = 0

while found != rule_count:
    for i in range(rule_count):
        if not rules_names[i]:
            candidates = []
            for rule in rules:
                if not rule in rules_names:
                    valid = True
                    (r1, r2) = rules[rule]
                    for passport in passports:
                        if not (r1[0] <= passport[i] <= r1[1] or r2[0] <= passport[i] <= r2[1]):
                            valid = False
                            break
                    if valid:
                        candidates.append(rule)
            if len(candidates) == 1:
                print(f'index {i} is {candidates[0]}')
                rules_names[i] = candidates[0]
                found += 1

print(rules_names)

tot = 1
for (i, name) in enumerate(rules_names):
    if name.startswith('departure'):
        tot *= my_passport[i]
        
print(tot)