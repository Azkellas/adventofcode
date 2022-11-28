import sys
sys.path.append("..")
from Amp import Amp

codes = open("input", "r")
codes = codes.read()
amp = Amp(codes)


commands = ["north",
    "take polygon",
    "north",
    "take astrolabe",
    "south",
    "south",
    "west",
    "take hologram",
    "north",
    "north",
    "take prime number",
    "south",
    "east",
    "take space law space brochure",
    "west",
    "south",
    "east",
    "south",
    "east",
    "take weather machine",
    "west",
    "south",
    "take manifold",
    "west",
    "take mouse",
    "north",
    "north"  # check point pour finir
]

history = ""
while True:
    res = amp.run()
    if not res:
        break
    res = chr(res)
    print(res, end="")
    history += res
    if history[-8:] == "Command?":
        if len(commands):
            inp = commands[0]
            print(inp)
            commands = commands[1:]
        else:
            inp = input()
        for l in inp:
            amp.inputs.append(ord(l))
        amp.inputs.append(10)