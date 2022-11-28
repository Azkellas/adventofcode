import sys
import re
rules = []

is_react = True
for line in sys.stdin:
    line = line.strip()
    if line == '':
        is_react = False
    
    elif is_react:
        line = line.split('=>')
        r_from = line[0].strip()
        r_to   = line[1].strip()
        rules.append((r_from, r_to))
        
    else:
        target = line.strip()


# see https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/

total = len(re.findall('[A-Z]', target))
Rn    =  len(re.findall('Rn', target))
Y     =  len(re.findall('Y', target))

print(total - 2*Rn - 2*Y - 1)