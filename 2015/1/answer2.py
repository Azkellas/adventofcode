line = input()
print(line.count('(') - line.count(')'))

pos = 0
for i in range(len(line)):
    c = line[i]
    if c == '(':
        pos += 1
    if c == ')':
        pos -= 1
    if pos < 0:
        print(i + 1)
        break