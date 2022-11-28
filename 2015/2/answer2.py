from sys import stdin

total = 0
for line in stdin:
    l, w, h = [int(i) for i in line.split('x')]
    total += l*w*h + 2*min(l+w, l+h, w+h)
print(total)
