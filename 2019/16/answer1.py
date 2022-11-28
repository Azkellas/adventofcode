data = [int(i) for i in input()]

print("".join([str(d) for d in data]))

for step in range(100):
    newData = []

    for ii in range(len(data)):
        i = ii+1
        pattern = [0] * i + [1] * i + [0] * i + [-1] * i

        res = 0
        for j in range(len(data)):
            res += data[j] * pattern[(j + 1)%len(pattern)]
            #print(f"{data[j]}*{pattern[(j + 1)%len(pattern)]}", end=", ")
        #print(f" = {res}")
        res = abs(res) % 10
        newData.append(res)
    
    data = newData
    print("".join([str(d) for d in data]))
