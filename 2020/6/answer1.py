import sys

total = 0

dict = {}

for line in sys.stdin:
    line = line.strip()
    print("line: ", line)
    char = ord('a')
    while char <= ord('z'):
        if chr(char) in line:
            dict[char] = True
        char += 1

    if line == '':
        print(len(dict))
        total += len(dict)
        dict = {}

print(total)