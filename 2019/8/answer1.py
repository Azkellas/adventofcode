data = input()

w = 25*6
bestScore = -1
best0 = w+1
i = 0
while i + w <= len(data):
    s = data[i:i+w]
    if s.count('0') < best0:
        best0 = s.count('0')
        bestScore = s.count('1') * s.count('2')
    i += w
print(best0, bestScore)