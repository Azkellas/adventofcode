import sys

sizes = [int(line) for line in sys.stdin]
count = len(sizes)

max_size = 150
valid = [0 for i in range(count)]

def rec(p_idx, p_size):
    if p_idx + 1 == count:
        return p_size == 0 or p_size == sizes[p_idx]

    tot = 0
    tot += rec(p_idx + 1, p_size)
    if p_size >= sizes[p_idx]:
        tot += rec(p_idx + 1, p_size - sizes[p_idx])

    return tot

    # tot = 0
    # max_s = p_size // sizes[p_idx]
    # if p_size % sizes[p_idx] == 0:
    #     max_s += 1
    # for i in range(max_s):
    #     valid[p_idx] = i
    #     tot += rec(p_idx + 1, p_size - i * sizes[p_idx])
    # return tot

print(rec(0, max_size))