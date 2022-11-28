import sys
sys.path.append("..")
from Amp import Amp
codes = input()
amp = Amp(codes)



# jump only if safe and a hole is nearby
str = '''NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
'''

for s in str:
    print(s, end="")
    amp.inputs.append(ord(s))

res = 'y'
while res:
    res = amp.run()
    if res:
        if res < 200:
            print(chr(res), end="")
        else:
            print(res)