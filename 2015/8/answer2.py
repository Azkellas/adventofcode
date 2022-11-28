from sys import stdin

tot = 0
for line in stdin:
    line = line[:-1]
    tot += 2 + line.count('"') + line.count('\\')
print(tot)