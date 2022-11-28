beg, end = [int(i) for i in input().split('-')]
print(beg, end)


def hasDouble(s):
    i = 0
    while i < len(s) - 1:
        if s[i] == s[i+1]:
            if i == len(s) - 2 or s[i] != s[i+2]:
                return True
            c = s[i]
            while s[i] == c and i < len(s) - 1:
                i += 1
        else:
            i += 1
    return False

def isInc(s):
    for i in range(len(s) - 1):
        if s[i+1] < s[i]:
            return False
    return True

tot = 0
while beg <= end:
    s = str(beg)
    val = hasDouble(s) and isInc(s)
    if val:
        tot += 1
        print(s)
    beg += 1
print(tot)