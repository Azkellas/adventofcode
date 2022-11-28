mem = [int(i) for i in input().split(',')]


last_seen = {}
prev_seen = {}

for i, val in enumerate(mem):
    last_seen[val] = i
l_input = len(mem)

maxi = 30000000
# maxi = 2020
value = mem[-1]

DEBUG = True
def debug(*args, **kwargs):
    if DEBUG: print(args, **kwargs)

for turn in range(l_input, maxi):
    # debug('  ', last_seen)
    # debug(value, end=': ')
    if value in prev_seen:
        # debug(f'2ce, {last_seen[value]} - {prev_seen[value]}')
        new_value = last_seen[value] - prev_seen[value]
        prev_seen[value] = last_seen[value]
        value = new_value
        if value in last_seen:
            prev_seen[value] = last_seen[value]
        last_seen[value] = turn
        last_seen[value] = turn
    else:
        # debug(f'1st, {last_seen[value]}')
        if 0 in last_seen:
            prev_seen[0] = last_seen[0]
        last_seen[0] = turn
        value = 0
print(value)