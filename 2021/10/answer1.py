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
score = 0
score_table = {')': 3, ']': 57, '}': 1197, '>': 25137}
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
                    print(line)
                    print(f"expected {C} got {c}")
                    score = score + score_table[c]
                    valid = False
    if valid:
        valid_lines.append(line)
print(score)