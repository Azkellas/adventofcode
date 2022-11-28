import sys



for line in sys.stdin:
    line = line.strip()
    convert = ""
    for c in line:
        convert = convert + bin(int(c, 16))[2:].zfill(4)
    print(line)
    print(convert)

    total = 0

    def get_ops(convert, depth=0):
        global total

        v = int(convert[:3], 2)
        t = int(convert[3:6], 2)

        total = total + v
    
        if t == 4:
            # literal value
            n = ""
            i = 6
            while True:
                st = convert[i:i+5]
                n = n + st[1:]
                i = i + 5
                if st[0] == "0":
                    break
            n = int(n, 2)
            # print(" "*depth + "literal = ", n)
            return {"type": "literal", "val": n, "length": i}

        else:
            i = convert[6]
            if i == '0':
                n = convert[7:7+15]
                n = int(n, 2)
                # print(" "*depth + "length in bits:", n)
                convert = convert[7+15:]
                length_seen = 0
                while True:
                    res = get_ops(convert, depth + 1)
                    convert = convert[res["length"]:]
                    length_seen = length_seen + res["length"]
                    # print(" "*depth + " seen ", length_seen)
                    if length_seen == n:
                        # print(" "*depth + "end of operator")
                        break
                return {"type": "ope", "val": 0, "length": 7+15+length_seen}
            if i == '1':
                n = convert[7:7+11]
                n = int(n, 2)
                # print(" "*depth + "length in ops:", n)

                convert = convert[7+11:]
                length_seen = 0
                for o in range(n):
                    # print(" "*depth + "convert", convert)
                    res = get_ops(convert, depth + 1)
                    convert = convert[res["length"]:]
                    length_seen = length_seen + res["length"]
                    # print(" "*depth + "  seen ", length_seen)
                return {"type": "ope", "val": 0, "length": 7+11+length_seen}


    get_ops(convert)

    print()
    print("total", total)