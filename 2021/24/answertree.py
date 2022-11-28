import sys
import copy
import re

###############################################################################
#                           Equation
###############################################################################

# Tree structure containing an equation
class Equation:
    def __init__(self, op="val", val=0):
        self.op = op
        self.val = val
        self.left = None
        self.right = None

        # used later to elague tree
        self.force_result = "noforce"

    # debug equation
    def __str__(self):
        if self.op == "val": return str(self.val)
        return f"({self.left} {self.op} {self.right})"

    def is_val(self, val):
        return self.op == "val" and self.val == val

# Default merge if no simplification is possible
def eq_merge(cmd, eq1, eq2):
    eq = Equation(cmd)
    eq.left, eq.right = eq1, eq2

    # make != since 2 == commands follow each other
    if cmd == "==" and eq2.is_val(0) and eq1.op == "==":
        eq = eq1
        eq.op = "!="
        return eq

    # try to simplify to a single val if possible
    try:
        eq.val, eq.op, eq.left, eq.right = eval(str(eq)), "val", None, None
    except Exception as e:
        pass
    
    return eq

# Function to handle /26 and %26 specific actions
def action_by_26(action, node):
    if node.op == "*":
        assert(node.right.is_val(26))
        if action == "divide":  return node.left
        if action == "mod":     return Equation()            

    elif node.op == "+":
        res = Equation("+")
        res.left = action_by_26(action, node.left)
        res.right = action_by_26(action, node.right)

        if res.op == "+" and res.left.is_val(0):    res = res.right
        if res.op == "+" and res.right.is_val(0):   res = res.left

        try:
            res.op, res.val = "val", eval(str(res))
        except Exception:
            pass
        return res

    elif node.op == "val":
        v = node.val
        if isinstance(v, int):
            if action == "divide":  return Equation(val=v//26)
            if action == "mod":     return Equation(val=v%26)
        else:
            if action == "divide":  return Equation()
            if action == "mod":     return Equation(val=v)            
    else:
        print("other op")
        print(node)
        exit(1)


###############################################################################
#                           Node
###############################################################################

# Tree structure containing a path in the script
# Each node represent a condition on our input
class Node:
    def __init__(self, condition="True"):
        self.condition = condition
        self.if_true = None
        self.if_false = None
        self.regs = {'x': Equation(), 'y': Equation(), 'z': Equation(), 'w': Equation()}

    def is_leaf(self):
        return self.condition == "True"

    def get_val(self, a):
        if a in 'xyzw': return self.regs[a]
        if a.isnumeric(): a = int(a)
        return Equation(val=a)


def apply(node, cmd, a, b):
    if node.is_leaf():
        val_b = node.get_val(b)

        if cmd == 'inp':
            node.regs[a] = Equation(val=val_b)

        if cmd == '+':
            if not val_b.is_val(0):
                # Ignore cases where a or b is 0, Add to a.right if it's an int value
                # Use default if no simplification
                if node.regs[a].is_val(0):
                    node.regs[a] = val_b
                elif node.regs[a].op == "+" and node.regs[a].right.op == "val":
                    node.regs[a].right.val = eval(f"{node.regs[a].right.val} + {val_b}")
                else:
                    node.regs[a] = eq_merge("+", node.regs[a], val_b)

        if cmd == '*':
            # If a or b is 0, set node to 0, if 1 to the other, use default if no simplification
            if val_b.is_val(0):
                node.regs[a] = Equation()
            elif node.regs[a].is_val(0):
                node.regs[a] = Equation()
            elif val_b.is_val(1):
                node.regs[a] = node.regs[a]
            elif node.regs[a].is_val(1):
                node.regs[a] = val_b
            else:
                node.regs[a] = eq_merge("*", node.regs[a], val_b)

        if cmd == '//':
            assert(val_b.is_val(26) or val_b.is_val(1))
            if val_b.is_val(26):
                res = action_by_26("divide", node.regs[a])
                node.regs[a] = res

        if cmd == '%':
            assert(val_b.is_val(26))
            res = action_by_26("mod", node.regs[a])
            node.regs[a] = res

        if cmd == '==':
            node.regs[a] = eq_merge("==", node.regs[a], val_b)

        # After command we end with a !=: time to branch
        if node.regs[a].op == "!=":
            # Set condition
            cond = str(node.regs[a])

            v = node.regs[a].left
            if v.op == "val" and isinstance(v.val, int):
                # Check conds of type NUMB != LETTER
                v = int(v.val)
                if not 1 <= v <= 9:
                    # The condition is a tautonomy (e.g. A != 50)
                    node.regs[a] = Equation(val=1)
                    return
            v = node.regs[a].right
            if v.op == "val" and isinstance(v.val, int):
                # Check conds of type LETTER != NUMB
                print(v)
                v = int(v.val)
                if not 1 <= v <= 9:
                    # The condition is a tautonomy (e.g. A != 50)
                    node.regs[a] = Equation(val=1)
                    return

            node.condition = cond

            # Init children
            node.if_true = Node()
            node.if_false = Node()
            node.if_true.regs = copy.deepcopy(node.regs)
            node.if_false.regs = copy.deepcopy(node.regs)
            node.if_true.regs[a] = Equation(val=1)
            node.if_false.regs[a] = Equation(val=0)
            node.regs = None

    else:
        apply(node.if_true,  cmd, a, b)
        apply(node.if_false, cmd, a, b)


###############################################################################
#                           Generate Node tree
###############################################################################

cmd_transcripts = {
    'add': '+',
    'mul': '*',
    'mod': '%',
    'eql': '==',
    'neq': '!=',
    'div': '//',
    'inp': 'inp'
}

n = "ABCDEFGHIJKLMNOP"
input_i = 0
root = Node()
for i, line in enumerate(sys.stdin):
    line = line.split()
    if line[0] == 'inp':
        line.append(n[input_i])
        input_i += 1

    cmd, a, b = line
    cmd = cmd_transcripts[cmd]

    print(i+1, a, cmd, b)
    apply(root, cmd, a, b)

###############################################################################
#                           Get valid path
###############################################################################

# Parse tree to only keep valid nodes
# A node is valid if there exist a set of conditions leading to a leaf with z = 0
def is_valid(node):
    if node.is_leaf():
        r = node.regs['z'].is_val(0)
        node.force_result = ["none", "z"][r]
        return r
    else:
        # == is possible only if the difference is less than 9
        re_f = re.findall("-?\d+", node.condition)
        both_cond = (abs(int(re_f[0])) <= 8)

        fa = is_valid(node.if_false)
        tr = is_valid(node.if_true)
        if both_cond and fa and tr:
            node.force_result = "both"
        elif both_cond and fa:
            node.force_result = "false"
        elif tr:
            node.force_result = "true"
        else:
            node.force_result = "none"
        return fa or tr
is_valid(root)

# make sure only one path is valid and print it
def dbg_paths(node, path=[]):
    if node.force_result == "z":
        print("conditions:")
        for x in path:
            if x[1]: print("  " + x[0])
            else: print("  " + x[0].replace('!=', '=='))
        return 1
    else:
        res = 0
        if node.force_result in ["both", "true"]:
            p = copy.deepcopy(path)
            p.append((node.condition, True))
            res = res + dbg_paths(node.if_true, p)
        if node.force_result in ["both", "false"]:
            p = copy.deepcopy(path)
            p.append((node.condition, False))
            res = res + dbg_paths(node.if_false, p)
        return res
print(f"valid path count = {dbg_paths(root)}")
