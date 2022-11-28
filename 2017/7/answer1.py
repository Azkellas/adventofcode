import sys
import re

class Node:
    def __init__(self, name):
        self.name = name
        self.weight = -1_000_000
        self.children = set()
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
        root.children.add(c)
        nodes[c].parent = root

    nodes[name] = root

for name in nodes:
    r = nodes[name]
    if r.parent == None and len(r.children):
        print("root = ", name)
