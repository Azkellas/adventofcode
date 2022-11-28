from collections import Counter
from sys import stdin

score = 0
for phrase in stdin:
    phrase = phrase.strip().split(' ')
    valid = True
    for i in range(len(phrase)):
        for j in range(len(phrase)):
            if i != j and Counter(phrase[i]) == Counter(phrase[j]):
                valid = False
    score += valid
print(score)
