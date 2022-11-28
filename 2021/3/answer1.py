import sys

lines = sys.stdin

ones   = []
zeroes = []
for line in lines:
    line = line.strip()
    if len(ones) == 0:
        l = len(line)
        ones = [0 for i in range(l)]
        zeroes = [0 for i in range(l)]
    for i in range(l):
        if line[i] == '1': ones[i] = ones[i] + 1
        else:              zeroes[i] = zeroes[i] + 1

gamma = ''
epsilon = ''
for i in range(l):
    if ones[i] > zeroes[i]:
        gamma = gamma + '1'
        epsilon = epsilon + '0'
    else:
        gamma = gamma + '0'
        epsilon = epsilon + '1'

print(gamma, epsilon)
print(int(gamma, 2), int(epsilon, 2))
print(int(gamma, 2) * int(epsilon, 2))