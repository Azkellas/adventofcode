line = input()


steps = 50
for step in range(steps):
    new_line = ''
    i = 0
    while i < len(line):
        c = line[i]
        i0 = i
        while i < len(line) and line[i] == c:
            i += 1
        new_line += str(i - i0) + c
    line = new_line
    # print(line)
print(len(line))