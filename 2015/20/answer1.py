target = int(input()) // 10



scores = [0 for i in range(target)]

max_val = 0

best_house = target - 1
for elf in range(1, best_house):
    house = elf
    while house <= best_house:
        scores[house] += 10 * elf

        if scores[house] > max_val:
            print(house, scores[house])
            max_val = scores[house]

        if scores[house] >= target * 10 and house < best_house:
            best_house = house
            print("new best: ", house)
            # exit()
        house += elf
