import sys
sys.path.append("..")
from Amp import Amp
codes = input()

amps = []
for i in range(50):
    amps.append(Amp(codes))
    amps[i].inputs.append(i)

NAT = []

working = [True for i in range(50)]
workings = 50
outputs = [[] for i in range(50)]
lastY = None
idlingFor = 0
while workings > 0:
    idling = 0
    for i in range(50):
        if len(amps[i].inputs) == 0:
            idling += 1

    for i in range(50):
        if working[i]:
            amp = amps[i]
            prevSize = len(amp.inputs)
            noPacket = False
            if len(amp.inputs) == 0:
                noPacket = True
                amp.inputs.append(-1)
            res = amp.runByStep()
            if noPacket:
                amp.inputs = []

            # if not noPacket and len(amp.inputs) < prevSize:
            #     print(f"amp {i} read a value")

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
                        print(f"NAT = ({x}, {y})")
                        NAT = [x, y]
                    else:
                        print(f"add packet to {nex} ({x}, {y})")
                        amps[nex].inputs.append(x)
                        amps[nex].inputs.append(y)
                    outputs[i] = []

    # print("idl", idling)
    if idling == 50:
        idlingFor += 1
    else:
        idlingFor = 0

    if idlingFor == 1000:
        print("idling -> nat")
        if len(NAT):
            amps[0].inputs.append(NAT[0])
            amps[0].inputs.append(NAT[1])
            print(amps[0].inputs)
            if lastY == NAT[1]:
                print("res = ", lastY)
                exit()
            lastY = NAT[1]
        else:
            print("stil wtf")