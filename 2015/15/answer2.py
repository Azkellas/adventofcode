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
max_cal = 500
range_i = max_cal // ing[0]['cal']
last_score = -1e15
for i in range(range_i):
    cal = i * ing[0]['cal']

    range_j = (max_cal - cal) // ing[1]['cal']
    for j in range(range_j):
        # if j == 0:
        #     continue # dura <= 0

        cal = i * ing[0]['cal'] + j * ing[1]['cal']

        range_k = (max_cal - cal) // ing[2]['cal']
        for k in range(range_k):
            # if 3 * i < 3 * j + k:
            #     continue # capa <= 0

            cal = i * ing[0]['cal'] + j * ing[1]['cal'] + k * ing[2]['cal']

            rem_cal = max_cal - cal
            if rem_cal % ing[3]['cal'] == 0:
                l = rem_cal // ing[3]['cal']
                # if k <= 2 * l:
                #     continue # fla <= 0
                
                # if 2 * i <= 3 * l:
                #     continue # tex <= 0

                if i + j + k + l == 100:
                    cap = i * ing[0]['cap'] + j * ing[1]['cap'] + k * ing[2]['cap'] + l * ing[3]['cap']
                    dur = i * ing[0]['dur'] + j * ing[1]['dur'] + k * ing[2]['dur'] + l * ing[3]['dur']
                    fla = i * ing[0]['fla'] + j * ing[1]['fla'] + k * ing[2]['fla'] + l * ing[3]['fla']
                    tex = i * ing[0]['tex'] + j * ing[1]['tex'] + k * ing[2]['tex'] + l * ing[3]['tex']
                    score = [0, cap][cap > 0] * [0, dur][dur > 0] * [0, fla][fla > 0] * [0, tex][tex > 0]

                    if score > max_score:
                        print(score, i, j, k, l, ':', cap, dur, fla, tex)
                        max_score = score

print(max_score)
