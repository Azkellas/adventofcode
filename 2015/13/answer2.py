import sys
from itertools import permutations

guests = []
scores = {}
for line in sys.stdin:
    line = line.split('.')[0]
    line = line.split()
    guest_1 = line[0]
    guest_2 = line[-1]
    positive = line[2] == 'gain'
    value = int(line[3]) * [-1, 1][line[2] == 'gain']

    if not guest_1 in guests:
        guests.append(guest_1)
        scores[guest_1] = {}
    
    scores[guest_1][guest_2] = value

scores['myself'] = {}
for guest in guests:
    scores[guest]['myself'] = 0
    scores['myself'][guest] = 0
guests.append('myself')
guest_count = len(guests)

best_score = -1e12

for combinaison in permutations(guests):
    # print(combinaison)
    score = 0
    for i in range(guest_count):
        score += scores[combinaison[(i  ) % guest_count]][combinaison[(i+1) % guest_count]]
        score += scores[combinaison[(i+1) % guest_count]][combinaison[(i  ) % guest_count]]
    if score > best_score:
        best_score = score

print(best_score)