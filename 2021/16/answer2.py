import sys
import math


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
            return {"val": n, "length": i}

        else:
            params = []
            global_length = 0
            i = convert[6]
            if i == '0':
                n = convert[7:7+15]
                n = int(n, 2)
                # print(" "*depth + "length in bits:", n)
                convert = convert[7+15:]
                length_seen = 0
                while True:
                    res = get_ops(convert, depth + 1)
                    params.append(res["val"])
                    convert = convert[res["length"]:]
                    length_seen = length_seen + res["length"]
                    # print(" "*depth + " seen ", length_seen)
                    if length_seen == n:
                        # print(" "*depth + "end of operator")
                        break
                global_length = 7+15+length_seen
            if i == '1':
                n = convert[7:7+11]
                n = int(n, 2)
                # print(" "*depth + "length in ops:", n)

                convert = convert[7+11:]
                length_seen = 0
                for o in range(n):
                    # print(" "*depth + "convert", convert)
                    res = get_ops(convert, depth + 1)
                    params.append(res["val"])
                    convert = convert[res["length"]:]
                    length_seen = length_seen + res["length"]
                    # print(" "*depth + "  seen ", length_seen)
                global_length = 7+11+length_seen

            if t == 0:
                return {"val": sum(params), "length": global_length}
            if t == 1:
                return {"val": math.prod(params), "length": global_length}
            if t == 2:
                return {"val": min(params), "length": global_length}
            if t == 3:
                return {"val": max(params), "length": global_length}
            if t == 5:
                return {"val": params[0] > params[1], "length": global_length}
            if t == 6:
                return {"val": params[0] < params[1], "length": global_length}
            if t == 7:
                return {"val": params[0] == params[1], "length": global_length}

    print(get_ops(convert)["val"])

    print()
    # print("total", total)