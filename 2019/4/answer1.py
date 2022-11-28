beg, end = [int(i) for i in input().split('-')]
print(beg, end)


def hasDouble(n):
    s = str(n)
    for i in range(len(s) - 1):
        if s[i] == s[i+1]:
            return True
    return False

def isInc(n):
    s = str(n)
    for i in range(len(s) - 1):
        if s[i+1] < s[i]:
            return False
    return True

tot = 0
while beg <= end:
    tot += hasDouble(beg) and isInc(beg)
    beg += 1
print(tot)