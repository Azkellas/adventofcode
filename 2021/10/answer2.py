import sys

lines = sys.stdin

def opening(c):
    return c in "<([{"
def closing(c):
    return c in ">)]}"
def get_closing(o):
    if o == '<': return '>'
    if o == '(': return ')'
    if o == '[': return ']'
    if o == '{': return '}'

valid_lines = []
for line in lines:
    line = line.strip()
    p = []
    valid = True
    for c in line:
        if not valid:
            break
        if opening(c):
            p.append(c)
        else:
            o = p[-1]
            if opening(o):
                C = get_closing(o)
                if c == C:
                    p.pop()
                else:
                    valid = False
    if valid:
        valid_lines.append(line)

scores = []
score_table = {')': 1, ']': 2, '}': 3, '>': 4}

for line in valid_lines:
    p = []
    for c in line:
        if opening(c):
            p.append(c)
        else:
            p.pop()
    s = 0
    while len(p) > 0:
        o = p.pop()
        c = get_closing(o)
        s = s * 5 + score_table[c]
    print(s)
    scores.append(s)

scores.sort()
print(scores[len(scores) // 2])