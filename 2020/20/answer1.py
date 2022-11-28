import sys
import math 

tiles = {}

tile_id = -1
tile_grid = []
for line in sys.stdin:
    line = line.strip()
    if line == '':
        print('tile ', tile_id)
        tiles[tile_id] = tile_grid
        tile_grid = []
    elif line.startswith('Tile '):
        tile_id = int(line[5:-1])
    else:
        tile_grid.append(line)

sides = ['N', 'E', 'S', 'W']

print(len(tiles))

def get_side(grid, rota, side, flip):
    if rota == -1 or flip == -1:
        print('err')
        exit()

    res = ''
    side = sides[(4 + sides.index(side) - rota // 90) % 4]

    if flip == 'V' and (side == 'N' or side == 'S'):
        side = sides[(sides.index(side) + 2) % 4]
    if flip == 'H' and (side == 'W' or side == 'E'):
        side = sides[(sides.index(side) + 2) % 4]

    if side == 'N':
        for i in range(10):
            res += grid[0][i]
    elif side == 'E':
        for i in range(10):
            res += grid[i][9]
    elif side == 'S':
        for i in range(10):
            res += grid[9][9 - i]
    elif side == 'W':
        for i in range(10):
            res += grid[9 - i][0]

    if flip != 'N':
        res = res[::-1]

    return res


affected  = [-1 for i in range(len(tiles))]
rotations = [-1 for i in range(len(tiles))]
flips     = [-1 for i in range(len(tiles))]

lll = int(math.sqrt(len(tiles)))

all_vals = {}

all_rotas = [0, 90, 180, 270]
all_flips = ['N', 'H', 'V']
for tile_id in tiles:
    grid = tiles[tile_id]
    all_vals[tile_id] = {}
    for rota in all_rotas:
        all_vals[tile_id][rota] = {}
        for side in sides:
            all_vals[tile_id][rota][side] = {}
            for flip in all_flips:
                all_vals[tile_id][rota][side][flip] = get_side(grid, rota, side, flip)

print('computed')

def is_valid(tile_id, rotation, idx, flip):
    abv_idx = idx - lll
    if abv_idx >= 0:
        abv_tile = tiles[affected[abv_idx]]
        match1 = all_vals[affected[abv_idx]][rotations[abv_idx]]['S'][flips[abv_idx]]
        match2 = all_vals[tile_id]          [rotation          ]['N'][flip]
        if match1 != match2[::-1]:
            return False

    lft_idx = idx - 1
    if (idx % lll) != 0 and lft_idx >= 0:
        lft_tile = tiles[affected[lft_idx]]
        match1 = all_vals[affected[lft_idx]][rotations[lft_idx]]['E'][flips[lft_idx]]
        match2 = all_vals[tile_id]          [rotation          ]['W'][flip]
        if match1 != match2[::-1]:
            return False

    return True


def can_link(id1, rot1, flip1, id2, rot2, flip2, side):
    oside = sides[(sides.index(side) + 2) % 4]
    match1 = all_vals[id1][rot1][ side][flip1]
    match2 = all_vals[id2][rot2][oside][flip2]
    return match1 == match2[::-1]


graphs = {}

ccc = 0
for id1 in tiles:
    graphs[id1] = {}
    for rot1 in all_rotas:
        graphs[id1][rot1] = {}
        for flip1 in all_flips:
            graphs[id1][rot1][flip1] = {}
            for side in sides:
                graphs[id1][rot1][flip1][side] = []
                for id2 in tiles:
                    for rot2 in all_rotas:
                        for flip2 in all_flips:
                            if id1 != id2 and can_link(id1, rot1, flip1, id2, rot2, flip2, side):
                                graphs[id1][rot1][flip1][side].append((side, id2, rot2, flip2))
                                ccc += 1
                                print(f'{side} {id1}({rot1},{flip1}) < - > {id2}({rot2},{flip2})')

print('total valid ', ccc)                                    

all_set = set()
for id in tiles:
    for rot in all_rotas:
        for flip in all_flips:
            all_set.add((id, rot, flip))

def find_valids(idx):
    abv_idx = idx - lll
    if abv_idx >= 0:
        valid_abv = set()
        set_abv = True
        vv = graphs[affected[abv_idx]][rotations[abv_idx]][flips[abv_idx]]['S']
        for v in vv:
            if not v[1] in affected:
                valid_abv.add((v[1], v[2], v[3]))
    else:
        set_abv = False

    lft_idx = idx - 1
    if (idx % lll) != 0 and lft_idx >= 0:
        valid_lft = set()
        set_lft = True
        vv = graphs[affected[lft_idx]][rotations[lft_idx]][flips[lft_idx]]['E']
        for v in vv:
            if not v[1] in affected:
                valid_lft.add((v[1], v[2], v[3]))
    else:
        set_lft = False

    if not set_abv and not set_lft:
        res = set()
        for val in all_set:
            if not val[0] in affected:
                res.add(val)
        return res

    if not set_abv:
        return valid_lft
    if not set_lft:
        return valid_abv 
    return set(valid_abv) & set(valid_lft)


max_failed = 0
def rec2(next_idx):
    global max_failed
    if next_idx == len(tiles):
        print('found a solution')
        tot = 1
        tot *= affected[0]
        tot *= affected[lll - 1]
        tot *= affected[len(tiles) - lll]
        tot *= affected[len(tiles) - 1]
        print(affected)
        print(rotations)
        print(flips)

        print('score = ', tot)
        exit()

    valids = find_valids(next_idx)
    found_one = False
    for val in valids:
        affected[next_idx] = val[0]
        rotations[next_idx] = val[1]
        flips[next_idx] = val[2]
        rec2(next_idx + 1)
        affected[next_idx] = -1
        rotations[next_idx] = -1
        flips[next_idx] = -1
        found_one = True
    
    if not found_one and next_idx > max_failed:
        max_failed = next_idx
        print('failed at ', next_idx)
        # for i in range(next_idx):
        #     print(f'  {affected[i]}({rotations[i]},{flips[i]})', end='')
        # print()


print('starting rec')
rec2(0)

def rec(next_idx):
    if next_idx == len(tiles):
        print('found a solution')
        tot = 1
        tot *= affected[0]
        tot *= affected[lll - 1]
        tot *= affected[len(tiles) - lll]
        tot *= affected[len(tiles) - 1]
        print('score = ', tot)
        exit()

    found_one = False
    for tile_idx in tiles:
        if not tile_idx in affected:
            tile = tiles[tile_idx]
            for rotation in [0, 90, 180, 270]:
                for flip in ['N', 'V', 'H']:
                    if is_valid(tile_idx, rotation, next_idx, flip):
                        affected[next_idx] = tile_idx
                        rotations[next_idx] = rotation
                        flips[next_idx] = flip
                        rec(next_idx + 1)
                        affected[next_idx] = -1
                        rotations[next_idx] = -1
                        flips[next_idx] = -1
                        found_one = True
    
    if not found_one:
        if affected[0] == 1951:
            print('failed at ', next_idx)
            for i in range(next_idx):
                print(f'  {affected[i]}({rotations[i]},{flips[i]})', end='')
            print()


# for idx1 in tiles:
#     for idx2 in tiles:
#         if idx1 != idx2:
#             for rot1 in [0, 90, 180, 270]:
#                 for rot2 in [0, 90, 180, 270]:
#                     match1 = get_side(tiles[idx1], rot1, 'E')
#                     match2 = get_side(tiles[idx2], rot2, 'W')

#                     if match1 == match2[::-1]:
#                         print('valid ', idx1, idx2, rot1, rot2)


# rec(0)