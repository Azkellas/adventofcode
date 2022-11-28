conditions = [
"((((M + 11) % 26) -  9) != N)",
"((((L + 14) % 26) -  2) != M)",
"((((K +  9) % 26) -  6) != L)",
"((((J + 11) % 26) -  6) != K)",
"((((I +  1) % 26) + 12) != J)",
"((((H +  6) % 26) + 14) != I)",
"((((G + 10) % 26) + 12) != H)",
"((((F +  5) % 26) - 14) != G)",
"((((E +  1) % 26) -  3) != F)",
"((((D + 11) % 26) + 14) != E)",
"((((C +  2) % 26) - 10) != D)",
"((((B + 11) % 26) + 13) != C)",
"((((A + 8)  % 26) + 15) != B)"
]

zz = "((((((((((((((((((((((((((((((((((A + 8) * 26) + (B + 11)) * 26) + (C + 2)) // 26) * 26) + (D + 11)) * 26) + (E + 1)) // 26) * 26) + (F + 5)) // 26) * 26) + (G + 10)) * 26) + (H + 6)) * 26) + (I + 1)) * 26) + (J + 11)) // 26) * 26) + (K + 9)) // 26) * 26) + (L + 14)) // 26) * 26) + (M + 11)) // 26) * 26) + (N + 2))"

n = [9 for i in range(14)]
seen = 0
while True:
    seen = seen + 1
    A,B,C,D,E,F,G,H,I,J,K,L,M,N = n

    if seen % 10_000 == 0:
        print(A,B,C,D,E,F,G,H,I,J,K,L,M,N)

    valid = True
    culprit = -1
    for i, c in enumerate(conditions):
        # print(i, c)
        if not eval(c):
            valid = False
            culprit = 13 - i
            break

    if valid:
        culprit = 20
        if eval(zz) == 0:
            print("found answer !!")
            print(n)
            for c in n:
                print(c, end="")
            print()
            exit(1)

    i = 13
    while True:
        if i > culprit:
            n[i] = 9
            i = i - 1
        elif n[i] > 1:
            n[i] = n[i] - 1
            break
        else:
            n[i] = 9
            i = i - 1
