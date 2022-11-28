from sys import stdin

arr = []
for line in stdin:
    arr.append(int(line))

idx = 0
step = 0
while idx >= 0 and idx < len(arr):
    step += 1
    off = arr[idx]
    arr[idx] += 1 if arr[idx] < 3 else -1
    idx += off
print(step)
# print(arr)