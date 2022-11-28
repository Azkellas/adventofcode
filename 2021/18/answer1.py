import sys
import math

class Node:
    def __init__(self, p_root=None):
        self.a = None
        self.b = None
        self.val = -1
        self.parent = p_root

    def is_val(self):
        return self.val != -1
    
    def is_pair(self):
        return not self.is_val()

    def is_end_pair(self):
        return self.is_pair() and self.a.is_val() and self.b.is_val()

    def debug(self):
        if self.is_val():
            return str(self.val)
        else:
            return f"[{self.a.debug()},{self.b.debug()}]"

    def __str__(self):
        return self.debug()

#------------------------------------------------------------------------------
#------------------------------------------------------------------------------
#------------------------------------------------------------------------------

def parse_snail(data, p_root=None, d=0):
    # print("    "*d + data)
    r = Node(p_root)
    if data.isnumeric():
        r.val = int(data)
        return r

    if data[0] != "[" or data[-1] != "]":
        print("erreur for", data)
    data = data[1:-1]


    co = 0
    for i in range(len(data)):
        if data[i] == "[":
            co = co + 1
        if data[i] == "]":
            co = co - 1
        if co == 0 and data[i] == ",":
            sys.stdout.flush()
            r.a = parse_snail(data[:i], r, d+1)
            r.b = parse_snail(data[i+1:], r, d+1)
            return r

def find_boundary(node, direction, found_branching=False):
    r = node.parent
    if r == None:
        return None

    if found_branching:
        if node.is_val():
            return node
        return find_boundary([node.a, node.b][direction == "left"], direction, True)

    else:
        target = [r.a, r.b][direction == "right"]
        # find branching or go up
        if target == node:
            return find_boundary(r, direction, False)
        else:
            return find_boundary(target, direction, True)    

can_explode = True
def check_explodes(snail, depth=0):
    global can_explode
    if snail.is_pair():
        if depth >= 4 and snail.is_end_pair() and can_explode:
            lval = find_boundary(snail, "left")
            rval = find_boundary(snail, "right")
            # print(f"explode {snail.debug()}")
            # print(lval, rval)

            can_explode = False
            if lval: lval.val = lval.val + snail.a.val
            if rval: rval.val = rval.val + snail.b.val
            snail.val = 0

            return
        check_explodes(snail.a, depth + 1)
        check_explodes(snail.b, depth + 1)

def check_splits(snail):
    if snail.is_pair():
        tot = False
        tot = tot or check_splits(snail.a)
        tot = tot or check_splits(snail.b)
        return tot

    if snail.is_val() and snail.val >= 10:
        # print(f"splits {snail}")
        v = snail.val
        r = snail.parent
        n = Node(r)
        n.a = Node(n)
        n.b = Node(n)
        n.a.val = math.floor(v / 2)
        n.b.val = math.ceil(v / 2)
        if r.a == snail: r.a = n
        if r.b == snail: r.b = n
        return True
    return False

def get_magnitude(snail):
    if snail.is_val():
        return snail.val
    
    return 3 * get_magnitude(snail.a) + 2 * get_magnitude(snail.b)

#------------------------------------------------------------------------------
#------------------------------------------------------------------------------
#------------------------------------------------------------------------------

root = None

for line in sys.stdin:
    snail = line.strip()

    new_snail = parse_snail(snail)

    if root is None:
        root = new_snail
        continue

    new_root = Node(None)
    new_root.a = root
    new_root.b = new_snail
    root.parent = new_root
    new_snail.parent = new_root
    root = new_root

    while True:
        can_explode = True
        splitted = False
        check_explodes(root)
        if can_explode:
            splitted = check_splits(root)
        if not splitted and can_explode:
            break
    # print(root)
print(get_magnitude(root))