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
all_flips = ['N', 'V', 'H']
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
    if id1 == id2:
        return False
    oside = sides[(sides.index(side) + 2) % 4]
    match1 = all_vals[id1][rot1][ side][flip1]
    match2 = all_vals[id2][rot2][oside][flip2]
    return match1 == match2[::-1]


graphs = {}

for id1 in tiles:
    graphs[id1] = {}
    for rot1 in all_rotas:
        graphs[id1][rot1] = {}
        for flip1 in all_flips:
            graphs[id1][rot1][flip1] = {}
            for side in ['S', 'E', 'N', 'W']:
                graphs[id1][rot1][flip1][side] = []
                for id2 in tiles:
                    for rot2 in all_rotas:
                        for flip2 in all_flips:
                            if can_link(id1, rot1, flip1, id2, rot2, flip2, side):
                                graphs[id1][rot1][flip1][side].append((side, id2, rot2, flip2))
                                    

total = 1
corners = 0
first_tile = -1
for id1 in tiles:
    valids = 0
    for side in sides:
        if len(graphs[id1][0]['N'][side]) > 0: valids += 1
    if valids == 2:
        corners += 1
        total *= id1
        first_tile = id1

print(f'{corners} corners: {total}')


print('first tile ',first_tile)
affected[0] = first_tile
flips[0] = 'N'

for rot in range(4):
    if       len(graphs[first_tile][rot * 90            ]['N'][side]) == 0 \
        and  len(graphs[first_tile][((rot - 1) % 4) * 90]['N'][side]) == 0:
        print('initial rot = ', 90 * rot)
        rotations[0] = 90 * rot
        break


idx = 1

grid_size = len(tiles)
line_size = int(math.sqrt(grid_size))

def get_next(p_idx):
    for tile in tiles:
        if not tile in affected:
            for rot in all_rotas:
                for flip in all_flips:
                    if p_idx % line_size == 0:
                        top_idx = p_idx - line_size
                        if can_link(affected[top_idx], rotations[top_idx], flips[top_idx], \
                            tile, rot, flip, 'S'):
                            return (tile, rot, flip)
                    else:
                        lft_idx = p_idx - 1
                        if can_link(affected[lft_idx], rotations[lft_idx], flips[lft_idx], \
                            tile, rot, flip, 'E'):
                            return (tile, rot, flip)

for idx in range(1, grid_size):
    (tile, rot, flip) = get_next(idx)
    affected[idx] = tile
    rotations[idx] = rot
    flips[idx] = flip



for y in range(line_size):
    for x in range(line_size):
        print(affected[y * line_size + x], end=' ')
    print()


f = open("savept1", "a")
for i in range(grid_size):
    f.write(f'{affected[i]} {rotations[i]} {flips[i]}\n')
f.close()


# affected  = [-1 for i in range(len(tiles))]
# rotations = [-1 for i in range(len(tiles))]
# flips     = [-1 for i in range(len(tiles))]

