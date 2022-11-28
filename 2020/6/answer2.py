import sys

total = 0

dict = {}
char = ord('a')
while char <= ord('z'):
    dict[char] = True
    char += 1

for line in sys.stdin:
    line = line.strip()

    if line != '':
        print("line: ", line)
        char = ord('a')
        while char <= ord('z'):
            if not chr(char) in line:
                dict[char] = False
            char += 1

    if line == '':
        char = ord('a')
        while char <= ord('z'):
            if dict[char]:
                total += 1
            char += 1
        dict = {}
        char = ord('a')
        while char <= ord('z'):
            dict[char] = True
            char += 1

print(total)