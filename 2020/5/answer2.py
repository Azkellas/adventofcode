import sys

mini, maxi = 0, 0
def binary_tree(step):
    global mini, maxi
    if step == 'F' or step == 'L':
        maxi = (mini + maxi) / 2
    elif step == 'B' or step == 'R':
        mini = (mini + maxi) / 2
    else:
        print("wtf")

seats = [False for i in range(1000)]

for line in sys.stdin:
    line = line.strip()
    mini, maxi = 0, 128
    for i in range(7):
        binary_tree(line[i])
    row = mini
    if maxi != mini + 1:
        print("error line")

    mini, maxi = 0, 8
    for i in range(3):
        binary_tree(line[7 + i])
    if maxi != mini + 1:
        print("error col")

    col = mini
    seat_id = int(row * 8 + col)
    seats[seat_id] = True

id = 1
while id < 998:
    if seats[id - 1] and seats[id + 1] and not seats[id]:
        print("id ", id)
    id += 1