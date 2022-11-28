import sys

ints = []
for line in sys.stdin:
    ints.append(int(line.strip()))


idx = 25
window = 25
while True:
    valid = False
    for i in range(window):
        for j in range(window):
            if i != j and ints[idx - 1 - i] + ints[idx - 1 - j] == ints[idx]:
                valid = True
                break
    
    if not valid:
        print(ints[idx])
        exit()
    idx += 1