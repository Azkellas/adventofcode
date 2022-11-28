from sys import stdin

tot = 0
for line in stdin:
    line = line[:-1]
    tot += len(line) - len(eval(line))
print(tot)
