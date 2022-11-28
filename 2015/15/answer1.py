import sys
import re
ints = []

ing = []
for line in sys.stdin:
    line = line.strip()
    print(line)
    reg = r'(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)'

    match = re.match(reg, line)
    ing.append({
        'cap': int(match.group(2)),
        'dur': int(match.group(3)),
        'fla': int(match.group(4)),
        'tex': int(match.group(5)),
        'cal': int(match.group(6))        
    })

max_score = 0
for i in range(101):
    for j in range(101-i):
        for k in range(101-i-j):
            l = 100-i-j-k
            cap = i * ing[0]['cap'] + j * ing[1]['cap'] + k * ing[2]['cap'] + l * ing[3]['cap']
            dur = i * ing[0]['dur'] + j * ing[1]['dur'] + k * ing[2]['dur'] + l * ing[3]['dur']
            fla = i * ing[0]['fla'] + j * ing[1]['fla'] + k * ing[2]['fla'] + l * ing[3]['fla']
            tex = i * ing[0]['tex'] + j * ing[1]['tex'] + k * ing[2]['tex'] + l * ing[3]['tex']
            score = [0, cap][cap > 0] * [0, dur][dur > 0] * [0, fla][fla > 0] * [0, tex][tex > 0]
            if score > max_score:
                print(i, j, k, l, ':', cap, dur, fla, tex)
                max_score = score

print(max_score)
