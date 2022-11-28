from sys import stdin

depths = {}
still = 0

dists = {}

letters = []
depths["COM"] = 0

YOU = ""
SAN = ""

links = []
for line in stdin:
    A, B = line.split(')')
    B = B[:-1]
    if not A in letters:
        letters.append(A)
    if not B in letters:
        letters.append(B)

    if B == "YOU":
        YOU = A
    if B == "SAN":
        SAN = A

    dists[A, B] = dists[B, A] = 1
    for L in letters:
        if L == A or L == B:
            continue
        if (L, A) in dists and (L, B) in dists:
            print("BOTH IN", dists[L,A], dists[L,B])
            continue

        if (L, A) in dists:
            dists[L, B] = dists[B, L] = dists[L, A] + 1
            # print(L, B, dists[L, B])
        elif (L, B) in dists:
            dists[L, A] = dists[A, L] = dists[L, B] + 1
            # print(L, A, dists[L, A])

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
            for L in letters:
                if L == A or L == B:
                    continue
                if (L, A) in dists and (L, B) in dists:
                    # sprint("BOTH IN", dists[L,A], dists[L,B])
                    continue

                if (L, A) in dists:
                    dists[L, B] = dists[B, L] = dists[L, A] + 1
                    # print(L, B, dists[L, B])
                elif (L, B) in dists:
                    dists[L, A] = dists[A, L] = dists[L, B] + 1
                    # print(L, A, dists[L, A])


print(sum(depths.values()))
print(dists["YOU", "SAN"])
print(dists[YOU, SAN])

# for L in letters:
#     if (L, "YOU") in dists:
#         print("YOU", L, dists[L, "YOU"])