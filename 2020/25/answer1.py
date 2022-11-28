date = 20201227

# def get_number(sub, loop_size):
#     v = 1
#     for i in range(loop_size):
#         v = (v * sub) % date
#     return v

# card_nb = get_number(v)


card_key = int(input())
card_loop = 0
v = 1
while v % 20201227 != card_key:
    v = (v * 7) % date
    card_loop += 1

print('card loop =', card_loop)

door_key = int(input())
door_loop = 0
v = 1
while v % 20201227 != door_key:
    v = (v * 7) % date
    door_loop += 1

print('door loop =', door_loop)


v = 1
for i in range(card_loop):
    v = (v * door_key) % date
print('encr =', v)

v = 1
for i in range(door_loop):
    v = (v * card_key) % date
print('encr =', v)