import sys
import re


desc = {}
asc  = {}

for line in sys.stdin:
    line = line.strip()
    reg = r'\w \w bags contain (\w \w \w bag(?:s)?:(, ))+.'
    reg = r'\w \w bags contain \d \w \w bags.'
    reg = r'(\d+ )?(\w+ \w+) bags?'

    # print("line:", line)
    bag_count = 0
    line_color = ''
    for m in re.finditer(reg, line):
        bag_count += 1
        # print(m.group(0))
        str_count = m.group(1)
        color = m.group(2)
        
        if str_count is None:
            line_color = color
            desc[line_color] = []
        else:
            count = int(str_count[:-1])
            desc[line_color].append((count, color))
            if not color in asc:
                asc[color] = []
            asc[color].append(line_color)

    assert(bag_count == line.count('bag'))


def included_bags(p_color):
    tot = 1
    for (count, color) in desc[p_color]:
        tot += count * included_bags(color)
    return tot

print(included_bags('shiny gold') - 1)