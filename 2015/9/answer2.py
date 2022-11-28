from sys import stdin

dists = {}
dists['NorthPole'] = {}

city_count = 0

for line in stdin:
    line = line.split()
    # Faerun to Norrath = 129
    city1 = line[0]
    city2 = line[2]
    dist = int(line[4])
    for city in [city1, city2]:
        if not city in dists:
            dists[city] = {}
            dists['NorthPole'][city] = 0
            city_count += 1

    dists[city1][city2] = dist
    dists[city2][city1] = dist


min_dist = 0

def dfs(p_from, p_path, p_dist):
    global min_dist

    if len(p_path) == city_count and p_dist > min_dist:
        min_dist = p_dist

    for city in dists[p_from]:
        if not city in p_path:
            p_path.append(city)
            dfs(city, p_path, p_dist + dists[p_from][city])
            p_path.pop()

dfs('NorthPole', [], 0)
print(min_dist)