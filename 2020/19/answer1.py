import sys
import re

rules = []
messages = []

for line in sys.stdin:
    line = line.strip()
    if ':' in line:
        [id, rule] = line.split(':')
        rule = rule.replace('"', '')
        rule = '( ' + rule.replace('|', ') | (') + ' )'
        rule = rule.split()
        if len(rule) == 3:
            rule = [rule[1]]
        valid = True
        for i, c in enumerate(rule):
            if c.isdigit():
                valid = False
                rule[i] = int(c)
        if valid:
            print('valid ', rule)
        rules.append({
            'id': int(id),
            'rule': rule,
            'valid': valid
        })

    elif len(line) != 0:
        messages.append(line)

rules.sort(key=lambda r: r['id'])

for rule in rules:
    print(rule)

while not rules[0]['valid']:
    for rule in rules:
        if not rule['valid']:
            valid = True
            for c in rule['rule']:
                if type(c) == int and not rules[c]['valid']:
                    valid = False
                    break
            if valid:
                rule['valid'] = True
                i = 0
                while i < len(rule['rule']):
                    c = rule['rule'][i]
                    if type(c) == int:
                        r = rule['rule']
                        r = r[:i] + ['('] + rules[c]['rule'] + [')'] + r[i+1:]
                        rule['rule'] = r
                    i += 1
                print('valid', rule['id'])

# for rule in rules:
#     print(rule['id'], rule['rule'])

# for rule in rules:
#     print(rule['id'], ''.join(rule['rule']))

rule = r'^' + ''.join(rules[0]['rule']) + r'$'
print(rule)
tot = 0
for msg in messages:
    if (re.match(rule, msg)):
        print(msg)
        tot += 1

print(tot)