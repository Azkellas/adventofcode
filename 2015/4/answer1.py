import hashlib

line = input()
print(line)
idx = 0
while True:
    byt = str.encode(line + str(idx))
    hash = hashlib.md5(byt).hexdigest()
    if hash[:5] == "00000":
        print(idx)
        break
    idx += 1
