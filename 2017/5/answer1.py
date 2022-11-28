from sys import stdin

arr = []
for line in stdin:
    arr.append(int(line))

idx = 0
step = 0
while idx >= 0 and idx < len(arr):
    step += 1
    arr[idx] += 1
    idx += arr[idx] - 1
print(step)