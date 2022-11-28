import sys


mol = input()
input()

cors = {}
for line in sys.stdin:
    print(line.strip())
    a, o, b = line.split()
    cors[a] = b
print(cors)

for step in range(10):
    nmol = ""
    for i in range(len(mol) - 1):
        s = mol[i:i+2]
        nmol = nmol + mol[i] + cors[s]
    nmol = nmol + mol[-1]
    mol = nmol
    print(step+1, len(mol))
maxi = -1
mini = 1_000_000
for c in mol:
    x = mol.count(c)
    if x > maxi: maxi = x
    if x < mini: mini = x
print(f"{maxi}-{mini} = {maxi-mini}")