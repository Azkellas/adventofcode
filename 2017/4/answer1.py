from collections import Counter
from sys import stdin

score = 0
for phrase in stdin:
    phrase = phrase.strip().split(' ')
    score += len(phrase) == len(set(phrase))
print(score)
