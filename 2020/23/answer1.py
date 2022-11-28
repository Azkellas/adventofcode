cups = input()

print(cups)


step_count = 100

for step in range(step_count):
    at = int(cups[0]) - 1
    rmv = cups[1:4]
    cups = cups[4:] + cups[0]
    print('step', step + 1)
    print('    ', cups, rmv, at)
    while not str(at) in cups:
        at -= 1
        if at <= 0:
            at = 9
            print(' rev', at)
    print('    dest ', at)
    at = cups.index(str(at)) + 1
    cups = cups[:at] + rmv + cups[at:]
    print('  ', cups)


at = cups.index('1')
res = cups[at+1:] + cups[:at]
print(res)
