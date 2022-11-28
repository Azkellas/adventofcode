data = input()

w = 25*6
bestScore = -1
best0 = w+1

img = [['2' for i in range(6)] for j in range(25)]
for i in range(len(data)):
    c = data[i]
    j = i % w
    y = j // 25
    x = j - y*25
    if img[x][y] == '2':
        img[x][y] = c

for y in range(6):
    for x in range(25):
        if img[x][y] == '1':
            print('x', end='')
        else:
            print(' ', end='')
    print()
print()