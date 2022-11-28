import sys

sizes = [int(line) for line in sys.stdin]
count = len(sizes)

max_size = 150
valid_path = [0 for i in range(count)]

max_used = count
score = 0

def rec(p_idx, p_size, p_used):
    global max_used, score

    if p_idx == count:
        valid = (p_size == 0)
        if valid:
            if p_used < max_used:
                max_used = p_used
                score = 0
            if p_used == max_used:
                score += 1
        return valid

    tot = 0
    valid_path[p_idx] = False
    tot += rec(p_idx + 1, p_size, p_used)
    if p_size >= sizes[p_idx]:
        valid_path[p_idx] = True
        tot += rec(p_idx + 1, p_size - sizes[p_idx], p_used + 1)

    return tot

rec(0, max_size, 0)
print(score)