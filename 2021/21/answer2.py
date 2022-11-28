import sys
import re 
import copy

scores =[0, 0]

pos = [0, 0]

for i in range(2):
    l = input().strip()
    c = re.findall("(\d+)", l)[1]
    pos[i] = int(c) - 1


class Memoize:
    def __init__(self, function):
        self._memory = {}
        self._function = function

    def __call__(self, player, pos, scores, depth):
        t = (player, pos[0], pos[1], scores[0], scores[1])
        if t not in self._memory:
            self._memory[t] = self._function(player, pos, scores, depth)
        return self._memory[t]

    def clear(self):
        self._memory = {}

@Memoize
def play(player, pos, scores, depth=0):
    s = [0, 0]
    for i in range(1, 4):
        for j in range(1, 4):
            for k in range(1, 4):
                npos = pos.copy()
                nscores = scores.copy()
                npos[player] = (npos[player] + i+j+k) % 10
                nscores[player] = min(21, nscores[player] + npos[player] + 1)

                if nscores[player] >= 21:
                    s[player] = s[player] + 1
                else:
                    r = play(1 - player, npos, nscores, depth + 1)
                    s[0] = s[0] + r[0]; s[1] = s[1] + r[1];
    return s

for i in range(100):
    print(max(play(0, pos.copy(), [0,0], 0)))
    print(len(play._memory))
    play.clear()