import sys

def is_junction(x):
    return x % 2 == 0 and 2 <= x <= 8

def get_room_name(x, y):
    if x == 2: return 'A'
    if x == 4: return 'B'
    if x == 6: return 'C'
    if x == 8: return 'D'
    print("getrooomane with",x,y)
    assert(False)
def get_room_x(name):
    if name == 'A': return 2
    if name == 'B': return 4
    if name == 'C': return 6
    if name == 'D': return 8
    print("getrooox with",name)
    assert(False)

def get_cost(name):
    if name == 'A': return 1
    if name == 'B': return 10
    if name == 'C': return 100
    if name == 'D': return 1000
    print("getcst with",name)
    assert(False)

def get_move_cost(grid, v, fn,fx, tn,tx, debug=False):
    added_cost = get_cost(v)
    if fn == 'H':
        fn,tn = tn,fn
        fx,tx = tx,fx
    if debug: print(f"{fx} + 1 + abs({tx} - {get_room_x([tn,fn][fn != 'H'])})")
    length = fx + 1 + abs(tx - get_room_x([tn,fn][fn != 'H']))
    added_cost = added_cost * length
    return added_cost

grid = {
    'H': ['.' for i in range(11)],
    'A': ['.', 'D', 'D', '.'],
    'B': ['.', 'C', 'B', '.'],
    'C': ['.', 'B', 'A', '.'],
    'D': ['.', 'A', 'C', '.']
}

y = -1
for line in sys.stdin:
    y = y + 1
    if 2 <= y <= 3:
        yy = [0, 3][y == 3]
        grid['A'][yy] = line[get_room_x('A') + 1]
        grid['B'][yy] = line[get_room_x('B') + 1]
        grid['C'][yy] = line[get_room_x('C') + 1]
        grid['D'][yy] = line[get_room_x('D') + 1]

print(grid)

def validate_room(grid, name):
    return grid[name][0] == name and grid[name][1] == name and grid[name][2] == name and grid[name][3] == name
def validate(grid):
    return validate_room(grid, 'A') and validate_room(grid, 'B') and validate_room(grid, 'C') and validate_room(grid, 'D')

def must_remove_room(grid, name):
    return grid[name][0] not in ['.', name] or grid[name][1] not in ['.', name] or grid[name][2] not in ['.', name] or grid[name][3] not in ['.', name]

def can_add_idx(grid, name):
    n = grid[name]
    if n[3] != name: return [-1, 3][n[3] == '.']
    if n[2] != name: return [-1, 2][n[2] == '.']
    if n[1] != name: return [-1, 1][n[1] == '.']
    if n[0] != name: return [-1, 0][n[0] == '.']
    return -1

def can_remove_idx(grid, name):
    if not must_remove_room(grid, name): return -1
    n = grid[name]
    for i in range(4):
        if n[i] != '.': return i

def dont_pop_out(grid, name):
    return grid[name][0] in ['.', name] and grid[name][1] in ['.', name] and grid[name][2] in ['.', name] and grid[name][3] in ['.', name]

class Memoize:
    def __init__(self, function):
        self._memory = {}
        self._function = function

    def __call__(self, grid, score, path):
        t = str(grid)
        if t not in self._memory or score < self._memory[t][0]:
            self._memory[t] = (score, self._function(grid, score, path))
        return self._memory[t][1]

    def clear(self):
        self._memory = {}

best_score_debug = 100_000
best_path = []

@Memoize
def bfs(grid, score, path):
    # print("                      ingridbs ", score)
    global best_score_debug
    if score > best_score_debug:
        return best_score_debug + 1
    if validate(grid):
        # print("ended with ", score)
        global best_path
        if score < best_score_debug:
            best_score_debug = score
            print("new best score of", score)
            best_path = path.copy()
            # for c in path:
            #     # print("  ", c)
            #     print("  ", c, get_move_cost(grid, c[0], c[1][0], c[1][1], c[2][0], c[2][1]))
        return score
    perfect_moves = []
    moves = []
    for x,v in enumerate(grid['H']):
        if v != '.':
            ti = can_add_idx(grid, v)
            if ti != -1:
                tx = get_room_x(v)
                delta = [-1, 1][tx > x]
                valid = True
                for j in range(x+delta, tx+delta, delta):
                    if grid['H'][j] != '.':
                        valid = False
                        break
                if valid:
                    perfect_moves.append((v, ('H', x), (v, ti), get_move_cost(grid, v, 'H', x, v, ti)))
    for name in ['A', 'B', 'C', 'D']:
        fi = can_remove_idx(grid, name)
        if fi != -1:
            fx = get_room_x(name)
            for tx in range(fx, -1, -1):
                if grid['H'][tx] != '.': break
                if not is_junction(tx):
                    moves.append((grid[name][fi], (name, fi), ('H', tx), get_move_cost(grid, grid[name][fi], name, fi, 'H', tx)))
            for tx in range(fx, 11, 1):
                if grid['H'][tx] != '.': break
                if not is_junction(tx):
                    moves.append((grid[name][fi], (name, fi), ('H', tx), get_move_cost(grid, grid[name][fi], name, fi, 'H', tx)))

    # perfect_moves.sort(key=lambda v : v[3])
    # moves.sort(key=lambda v : v[3])
    # perfect_moves.sort(key=lambda v : v[3], reverse=True)
    # moves.sort(key=lambda v : v[3],reverse=True)
    perfect_moves.extend(moves)

    best_score = 1_000_000
    for v, (fn, fx),  (tn, tx), cost in perfect_moves:
        grid[fn][fx] = '.'
        grid[tn][tx] = v
        path.append((v, (fn, fx), (tn, tx), cost))
        best_score = min(best_score, bfs(grid, score + cost, path))
        path.pop()
        grid[tn][tx] = '.'
        grid[fn][fx] = v
    return best_score

res = bfs(grid, 0, [])
print(res)
print(best_score_debug)
print(best_path)

def debug_grid(grid):
    print('#'*13)
    print('#', end='')
    for c in grid['H']: print(c, end='')
    print('#')
    for y in range(2):
        print(f"  #{grid['A'][y]}#{grid['B'][y]}#{grid['C'][y]}#{grid['D'][y]}#  ")

for v, (fn,fx),(tn,tx), cost in best_path:
    debug_grid(grid)
    print(f"move {v} from {fn,fx} to {tn,tx} costs {cost}")
    print()
    grid[tn][tx] = grid[fn][fx]
    grid[fn][fx] = '.'

print(res)
# print("reverse")
