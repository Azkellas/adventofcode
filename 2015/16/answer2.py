import sys
import re

crible = {
    'children': 3,
    'cats': 7,
    'samoyeds': 2,
    'pomeranians': 3,
    'akitas': 0,
    'vizslas': 0,
    'goldfish': 5,
    'trees': 3,
    'cars': 2,
    'perfumes': 1
}

data = [
    'children',
    'cats',
    'samoyeds',
    'pomeranians',
    'akitas',
    'vizslas',
    'goldfish',
    'trees',
    'cars',
    'perfumes'
]


for line in sys.stdin:
    nbr = int(re.search('Sue (\d+):', line).group(1))

    valid = True
    for key in data:
        val = re.search(key + ': (\d+)', line)
        if val:
            val = int(val.group(1))

            if key == 'cats' or key == 'trees':
                if val < crible[key]:
                    valid = False
                    break

            elif key == 'pomeranians' or key == 'goldfisg':
                if val > crible[key]:
                    valid = False
                    break

            else:
                if val != crible[key]:
                    valid = False
                    break

    if valid:
        print(nbr)
        # exit()