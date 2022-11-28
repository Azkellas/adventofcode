import sys
import re
from collections import Counter

class Node:
    def __init__(self, name):
        self.name = name
        self.weight = -1_000_000
        self.children = []
        self.parent = None

    def __hash__(self):
        return hash(self.name)

nodes = {}

# reg = "(\w{4}) \((\d+)\)(?: -> (\w{4} ))?"
# reg = "(\w{4}) \((\d+)\)(?: -> (\1,? ?)+)?"
for line in sys.stdin:
    line = line.strip().replace(',','') + ' '
    # print(line)
    g = re.findall("\w+|\d+", line)
    name = g[0]
    weight = int(g[1])
    children = g[2:]
    # print(root, weight, children)

    if name in nodes: root = nodes[name]
    else: root = Node(name)
    root.weight = weight

    for c in children:
        if c not in nodes:
            nodes[c] = Node(c)
        root.children.append(c)
        nodes[c].parent = root

    nodes[name] = root

root = None
for name in nodes:
    r = nodes[name]
    if r.parent == None and len(r.children):
        print("root = ", name)
        root = r

def get_weight(node):
    r = node.weight
    for c in node.children:
        r = r + get_weight(nodes[c])
    return r

def find_culprit(node):
    weights = []
    for c in node.children:
        weights.append(get_weight(nodes[c]))
    
    if weights.count(weights[0]) == len(weights):
        print(f"culprit is {node.name} with weight {node.weight}")
        return sum(weights)
    else:
        c = Counter(weights)
        resss = 0
        culprit = None
        target = 0
        for a in c:
            if c[a] == 1:
                for i in range(len(node.children)):
                    if weights[i] == a:
                        culprit = node.children[i]
                        resss = find_culprit(nodes[node.children[i]])
            else:
                target = a
        print(f"weight needed to be {target - resss} for {culprit}")
        return resss


find_culprit(root)