target = int(input()) // 10



scores = [0 for i in range(target)]

max_val = 0

best_house = target - 1
for elf in range(1, best_house):
    for i in range(50):
        house = elf * (i + 1)
        if house >= target:
            break
        scores[house] += 11 * elf

        if scores[house] > max_val:
            print(house, scores[house])
            max_val = scores[house]

        if scores[house] >= target * 10 and house < best_house:
            best_house = house
            print("new best: ", house)
            # exit()
        house += elf

print(best_house)