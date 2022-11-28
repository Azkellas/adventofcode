cups = [int(i) for i in list(input())]
print(cups)

class Node:
    def __init__(self):
        val = 0
        prev = 0
        next = 0

length = len(cups)
nodes = [Node() for i in range(length)]
for i in range(length):
    nodes[i].val = cups[i]
    nodes[i].prev = nodes[(length + i - 1) % length]
    nodes[i].next = nodes[(i + 1) % length]
    if nodes[i].val == 1:
        node1 = nodes[i]

prev_node = nodes[-1]
root = nodes[0]

for i in range(10, 1_000_001):
    node = Node()
    node.val = i
    prev_node.next = node
    node.prev = prev_node
    root.prev = node
    node.next = root
    prev_node = node
    nodes.append(node)

nodes.sort(key=lambda n: n.val)
for i in range(1_000_000):
    if nodes[i].val != i+1:
        print('error')
        exit()

print(len(nodes))

print('root val', root.val)

step_count = 10_000_000
for step in range(step_count):
    if step % 100_000 == 0:
        print(step)
    at = root.val - 1 if root.val > 1 else 1_000_000
    rmv = [root.next, root.next.next, root.next.next.next]

    root.next = rmv[-1].next
    rmv[-1].next.prev = root

    rmv_at = [n.val for n in rmv]
    while at in rmv_at:
        at -= 1
        if at <= 0:
            at = 1_000_000
    # print(at)

    node_at = nodes[at - 1]

    node_at.next.prev = rmv[-1]
    rmv[-1].next = node_at.next
    rmv[0].prev = node_at
    node_at.next = rmv[0]

    root = root.next

# n = node1.next
# while n.val != 1:
#     print(n.val, end='')
#     n = n.next
# print()

print(node1.next.val, node1.next.next.val)
print(node1.next.val * node1.next.next.val)
