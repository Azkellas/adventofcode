import re
import sys

ingredients = set()
allergens = set()
save_all_sets = {}
all_sets = {}

lines = []
lines_ing = []

reg = r'(.*) \(contains (.*)\)'
for line in sys.stdin:
    line = line.strip().replace(',', '')
    lines.append(line)

    match = re.match(reg, line)
    ing = match.group(1).split()
    lines_ing.append(match.group(1))
    ale = match.group(2).split()
    for i in ing:
        ingredients.add(i)
    for a in ale:
        allergens.add(a)
    
    s = set(ing)
    for a in ale:
        save_all_sets[a] = s
        if not a in all_sets:
            all_sets[a] = s
        else:
            all_sets[a] = all_sets[a].intersection(s)

valids = []

while len(valids) != len(all_sets):
    valid = 0
    for s in all_sets:
        if not s in valids and len(all_sets[s]) == 1:
            valids.append(s)
            for i in all_sets[s]:
                for q in all_sets:
                    if not q in valids and i in all_sets[q]:
                        all_sets[q].remove(i)

print(all_sets)

bad_ing = set()

for s in all_sets:
    for i in all_sets[s]:
        bad_ing.add(i)

print(bad_ing)

good_ing = set()

for i in ingredients:
    if not i in bad_ing:
        good_ing.add(i)

print(good_ing)

tot = 0
for line in lines_ing:
    for ing in line.split():
        tot += ing in good_ing


print(tot)