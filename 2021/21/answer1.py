import sys
import re 

scores =[0, 0]

pos = [0, 0]

dice = 0

for i in range(2):
    l = input().strip()
    c = re.findall("(\d+)", l)[1]
    pos[i] = int(c) - 1

player = 0
rolldices = 0

while scores[0] < 1000 and scores[1] < 1000:
    rolldices = rolldices + 3
    m = (dice + dice + 1 + dice + 2) + 3
    dice = (dice + 3) % 1000
    pos[player] = (pos[player] + m) % 10
    scores[player] = scores[player] + pos[player] + 1
    player = 1 - player
print(scores)
print(rolldices * min(scores))