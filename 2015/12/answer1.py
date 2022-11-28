import json

raw = input()
data = json.loads(raw)

total = 0

def dfs(node):
    global total

    if type(node) is dict:
        for child in node:
            if type(child) is int:
                total += child
            dfs(node[child])
    
    if type(node) is list:
        for el in node:
            dfs(el)
    
    if type(node) is int:
        total += node

dfs(data)
print(total)