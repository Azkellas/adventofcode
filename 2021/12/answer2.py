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

def rec(cave, visited, way, time_for_small):
    global paths
    if cave == "end":
        paths = paths + 1
        print(way)
        return
    for n in links[cave]:
        if (time_for_small and visited[n] < 2) or (visited[n] == 0) or is_big(n):
            if n == "start":
                continue
            visited[n] = visited[n] + 1
            if is_small(n) and visited[n] == 2: time_for_small = False
            way.append(n)
            rec(n, visited, way, time_for_small)
            if is_small(n) and visited[n] == 2: time_for_small = True
            visited[n] = visited[n] - 1
            way.pop()

visited = {}
for a in links:
    visited[a] = 0

way = []
way.append("start")
visited["start"] = True
rec("start", visited, way, True)

print(paths)