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
for passport in passports:
    for val in passport:
        valid = False
        for rule in rules:
            for r in rules[rule]:
                if r[0] <= val <= r[1]:
                    valid = True
        if not valid:
            print('invalid ', val)
            tot += val

print(tot)