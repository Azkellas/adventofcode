import sys
sys.path.append("..")
from Amp import Amp
codes = input()

amps = []
for i in range(50):
    amps.append(Amp(codes))
    amps[i].inputs.append(i)

working = [True for i in range(50)]
workings = 50
outputs = [[] for i in range(50)]
while workings > 0:
    for i in range(50):
        if working[i]:
            amp = amps[i]
            for k in range(1000):
                amp.inputs.append(-1)
            res = amp.runByStep()
            try:
                amp.inputs = amp.inputs[:amp.inputs.index(-1)]
            except:
                None
            
            if res == "END":
                working[i] = False
                workings -= 1
            elif res == "HALT":
                print("halt!")
            elif res:
                outputs[i].append(res)
                if len(outputs[i]) == 3:
                    [nex, x, y] = outputs[i]
                    if nex == 255:
                        print("res = ", y)
                        exit()
                    else:
                        print(f"add packet to {nex} ({x}, {y})")
                        amps[nex].inputs.append(x)
                        amps[nex].inputs.append(y)
                        outputs[i] = []
