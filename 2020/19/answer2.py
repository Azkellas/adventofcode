import sys
import re

rules = []
messages = []

for line in sys.stdin:
    line = line.strip()
    if ':' in line:
        [id, rule] = line.split(':')
        if id == '8':
            rule = '42 | 42 8'
        if id == '11':
            rule = '42 31 | 42 11 31'

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

        rule = {
            'id': int(id),
            'rule': rule,
            'valid': valid
        }
        rules.append(rule)

    elif len(line) != 0:
        messages.append(line)

rules.sort(key=lambda r: r['id'])

while not rules[0]['valid']:
    new_valid = False
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
                    # print('  a', rule['rule'])
                new_valid = True

    if not new_valid:
        break
                    # print('  a', rule['rule'])
 
for rule in rules:
    if not rule['valid']:
        print('invalid ', rule)

rule31 = '(' + ''.join(rules[31]['rule']) + ')'
rule31 = rule31.replace('(a)', 'a')
rule31 = rule31.replace('(b)', 'b')

rule42 = '(' + ''.join(rules[42]['rule']) + ')'
rule42 = rule42.replace('(a)', 'a')
rule42 = rule42.replace('(b)', 'b')

rule8 = '(' + rule42 + ')+'
rule11 = '(?:' + '|'.join(f'{rule42}{{{n}}}{rule31}{{{n}}}' for n in range(1, 10)) + ')'
rule = rule8 + rule11

print(rule31)
print(rule42)


tot = 0
for msg in messages:
    if (re.fullmatch(rule, msg)):
        print(msg)
        tot += 1

print(tot)