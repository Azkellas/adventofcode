arr = [int(i) for i in input()]
sc = 0

l = len(arr)

for i in range(len(arr)):
    if arr[i] == arr[ (i + l//2) % l]:
        sc += arr[i]

print(sc)