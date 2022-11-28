import sys

sdeck1 = []
sdeck2 = []

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
        if p1: sdeck1.append(a)
        else:  sdeck2.append(a)

print(sdeck1, sdeck2)


def print_winner(deck):
    print('WINNER ')
    tot = 0
    l = len(deck)
    for i in range(0, l):
        tot += deck[l-1-i] * (i+1)
    print(tot)
    exit()

def play_turn(deck1, deck2, seen, r):
    if (deck1, deck2) == seen:
        print('Wiin already')
        return 'OVER', deck1, deck2

    a = deck1[0]
    b = deck2[0]
    deck1 = deck1[1:]
    deck2 = deck2[1:]


    if a <= len(deck1) and b <= len(deck2):
        #recurse
        # d1 = deck1[:a]
        # d2 = deck2[:b]
        # s = []
        print('      playing recursive')
        # res = recursive_game(d1, d2, s, 1, r + 1)
        print('      ', deck1[:a], deck2[:b])
        res = recursive_game(deck1[:a], deck2[:b], (deck1[:a], deck2[:b]), 1, r + 1)
        if res:
            deck1 += [a, b]
            print(' ' * 3 * r, 'Player 1 wins: recursive')
            return True, deck1, deck2
        else:
            deck2 += [b, a]
            print(' ' * 3 * r, 'Player 2 wins: recursive')
            return False, deck1, deck2
    else:
        if a > b:
            deck1 += [a, b]
            print(' ' * 3 * r, 'Player 1 wins: lack of cards')
            return True, deck1, deck2
        else:
            deck2 += [b, a]
            print(' ' * 3 * r, 'Player 2 wins: lack of cards')
            return False, deck1, deck2

def recursive_game(d1, d2, s, i, r):
    while len(d1) and len(d2):
        print(' ' * 2 * (r-1), r, 'Round', i)
        res, d1, d2 = play_turn(d1, d2, s, r)
        if res == 'OVER':
            break
        print(' ' * 3, d1, d2)
        i += 1
    
    if r == 1:
        global sdeck1, sdeck2
        sdeck1 = d1
        sdeck2 = d2
    return res == 'OVER' or len(d1) > 0

recursive_game(sdeck1, sdeck2, (sdeck1, sdeck2), 1, 1)

print(sdeck1, sdeck2)

winnerdeck = sdeck1 if len(sdeck1) else sdeck2
tot = 0
l = len(winnerdeck)
for i in range(0, l):
    tot += winnerdeck[l-1-i] * (i+1)
print(tot)