arr = [int(i) for i in input()]
sc = 0

for i in range(len(arr)-1):
    if arr[i] == arr[i+1]:
        sc += arr[i]

if arr[0] == arr[-1]:
    sc += arr[0]

print(sc)