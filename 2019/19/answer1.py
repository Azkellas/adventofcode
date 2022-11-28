import sys
sys.path.append("..")
from Amp import Amp
codes = input()
amp = Amp(codes)

grid = [ [0 for i in range(50)] for j in range(50)]

for x in range(50):
    for y in range(50):
        amp = Amp(codes)
        amp.inputs.append(x)
        amp.inputs.append(y)
        res = amp.run()
        grid[x][y] = res


for y in range(50):
    for x in range(50):
        print(['.', '#'][grid[x][y]], end="")
    print()


tot = 0
for y in range(50):
    for x in range(50):
        tot += (grid[x][y] != 0)

print("tot = ", tot)