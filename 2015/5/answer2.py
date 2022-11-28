from sys import stdin
import re

from string import ascii_lowercase

total = 0
for line in stdin:
    line = line[:-1]
    length = len(line)


    okDouble = False
    dou = ''
    for a in ascii_lowercase:
        for b in ascii_lowercase:
            count = 0
            idx = 0
            while idx < length - 1:
                if line[idx] == a and line[idx+1] == b:
                    count += 1
                    idx += 1
                    if count > 1:
                        okDouble = True
                        dou = a+b
                        break
                idx += 1
            if okDouble:
                break
        if okDouble:
            break

    okPal = False
    pal = ''
    idx = 0
    while idx < length - 2:
        if line[idx] == line[idx+2]:
            okPal = True
            pal = line[idx:idx+3]
            break
        idx += 1

    if okDouble and okPal:
        total += 1
        print(line, end=": ")
        print('double: ', dou, end=" | ")
        print('pal: ', pal, end="")
        print()
print(total)
