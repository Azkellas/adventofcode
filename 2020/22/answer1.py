import sys

deck1 = []
deck2 = []

p1 = True
for line in sys.stdin:
    line = line.strip()
    if line == '':
        continue
    elif line.startswith('Player'):
        if '2' in line:
            p1 = False
    else:
        a = int(line)
        if p1: deck1.append(a)
        else:  deck2.append(a)

print(deck1, deck2)

while len(deck1) and len(deck2):
    a = deck1[0]
    b = deck2[0]
    deck1 = deck1[1:]
    deck2 = deck2[1:]
    if a > b:
        deck1 += [a, b]
    else:
        deck2 += [b, a]
    print(deck1, deck2)

winnerdeck = deck1 if len(deck1) else deck2
tot = 0
l = len(winnerdeck)
for i in range(0, l):
    tot += winnerdeck[l-1-i] * (i+1)
print(tot)