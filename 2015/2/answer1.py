from sys import stdin

total = 0
for line in stdin:
    l, w, h = [int(i) for i in line.split('x')]
    lw, lh, wh = l*w, l*h, w*h
    total += 2*lw + 2*lh + 2*wh + min(lw, lh, wh)
print(total)
