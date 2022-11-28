import sys

ints = []
for line in sys.stdin:
    ints.append(int(line.strip()))


goal = 25918798

count = len(ints)
for i in range(count):
    j = 0
    sum = 0
    while sum < goal:
        sum += ints[i+j]
        j += 1
    if sum == goal:
        smallest = 1e12
        largest = 0
        for k in range(j):
            if ints[i+k] < smallest:
                smallest = ints[i+k]
            if ints[i+k] > largest:
                largest = ints[i+k]
            
        print(smallest + largest)
        exit()
