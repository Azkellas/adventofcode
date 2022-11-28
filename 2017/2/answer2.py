from sys import stdin

tot = 0

for line in stdin:
    arr = [int(i) for i in line.split('\t')]
    s = 1
    for i in arr:
        for j in arr:
            if i != j and max(i,j) // min(i,j) == max(i,j) / min(i,j):
                s = max(i,j) // min(i,j)
    tot += s
print(tot)