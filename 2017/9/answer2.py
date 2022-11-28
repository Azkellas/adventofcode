import sys
import re

sumscore = 0
for line in sys.stdin:
    line = line.strip()
    print(line)
    
    # remove !
    line = re.sub("!.", "", line)

    # remove comments
    while True:
        s = ""
        comment = False
        for i,c in enumerate(line):
            if comment and c == ">":
                comment = False
                continue
            if not comment and c == "<":
                found = True
                comment = True
                continue
            if not comment:
                s = s + c
            else:
                sumscore = sumscore + 1
        
        line = s

        if line.find("<") == -1: break
    
    # count
    depth = 0
    score = 0
    for c in line:
        if c == "{": depth = depth + 1
        if c == "}":
            score += depth
            depth = depth - 1
    if depth != 0:
        print("depth issue")
    # sumscore = sumscore + score
print(sumscore)