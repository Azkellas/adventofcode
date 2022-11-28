import sys

lines = sys.stdin

def is_big(a):
    return a.isupper()
def is_small(a):
    return a.islower()

links = {}
for line in lines:
    line = line.strip()
    a, b = line.split("-")
    if a not in links: links[a] = set()
    if b not in links: links[b] = set()
    links[a].add(b)
    links[b].add(a)
print(links)

paths = 0

def rec(cave, visited, way):
    global paths
    if cave == "end":
        paths = paths + 1
        print(way)
        return
    for n in links[cave]:
        if not visited[n] or is_big(n) or n == "end":
            visited[n] = True
            way.append(n)
            rec(n, visited, way)
            visited[n] = False
            way.pop()

visited = {}
for a in links:
    visited[a] = False

way = []
way.append("start")
visited["start"] = True
rec("start", visited, way)

print(paths)