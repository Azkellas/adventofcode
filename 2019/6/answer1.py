from sys import stdin

depths = {}
still = 0

depths["COM"] = 0

links = []
for line in stdin:
    A, B = line.split(')')
    B = B[:-1]
    # print(A, B)
    if A in depths:
        depths[B] = depths[A] + 1
    else:
        # depths[B] = -1
        still += 1
        links.append((A, B))

while still > 0:
    for link in links:
        A, B = link
        # print(A, B)

        if A in depths and not B in depths:
            depths[B] = depths[A] + 1
            still -= 1
    # print(still)


print(sum(depths.values()))