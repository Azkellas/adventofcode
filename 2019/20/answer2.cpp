#include <iostream>
#include <cstring>
#include <vector>
#include <queue>
#include <unordered_map>

template <typename T>
using matrix = std::vector<std::vector<T>>;

int WIDTH, HEIGHT;

int DISTS[100][100];
int INF = 1'000'000;

std::unordered_map<int, int> hashTable;

class Point {
 public:
    int x {-1}, y{-1};
    Point() = default;
    Point(int _x, int _y) : x(_x), y(_y) {}


    inline bool isInside() {
        return x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT;
    }
};


int INNER = 0;
int OUTER = 1;
class Portal {
 public:
    int id {-1};
    std::string name {""};
    int type {INNER};
    Point entry;
    Point exit;
    int exitId {-1};
};



std::ostream& operator<<(std::ostream& os, Point const& p) {
    os << "(" << p.x << ", " << p.y << ")";
    return os;
}

std::ostream& operator<<(std::ostream& os, Portal const& p) {
    os << p.id << ": " << p.name << "  " << (p.type == INNER ? "INNER" : "OUTER") << " " << p.entry << " -> " << p.exit << "(" << p.exitId << ")";
    return os;
}


bool operator==(Point const& p, Point const& q) {
    return p.x == q.x && p.y == q.y;
}

bool operator!=(Point const& p, Point const& q) {
    return !(p == q);
}

Point operator+(Point const& p, Point const& q) {
    return {p.x + q.x, p.y + q.y};
}


Point directions[4] = {Point(-1, 0), Point(0, -1), Point(1, 0), Point(0, 1)};


void bfs(matrix<char> const& grid, std::vector<Portal> portals, Portal const& start) {
    int depths[WIDTH][HEIGHT];
    std::memset(depths, INF, sizeof(depths[0][0]) * WIDTH * HEIGHT);
    for (int x = 0; x < WIDTH; ++x) {
        for (int y = 0; y < HEIGHT; ++y) {
            depths[x][y] = INF;
        }
    }

    Point const& entry = start.entry;
    std::queue<Point> q;
    q.push(entry);
    depths[entry.x][entry.y] = 0;

    while (!q.empty()) {
        Point loc = q.front();
        q.pop();

        // check direct neighbours
        for (auto& dir : directions) {
            Point neigh = loc + dir;
            if (neigh.isInside() && grid[neigh.x][neigh.y] == '.') {
                if (depths[neigh.x][neigh.y] > depths[loc.x][loc.y] + 1) {
                    depths[neigh.x][neigh.y] = depths[loc.x][loc.y] + 1;

                    if (grid[neigh.x][neigh.y] == '.') {
                        // empty cell
                        q.push(neigh);
                    }
                }
            }
        }
    }

    for (auto& p : portals) {
        DISTS[start.id][p.id] = depths[p.entry.x][p.entry.y];
        if (DISTS[start.id][p.id] != INF && DISTS[start.id][p.id] != 0) {
            std::cerr << "  | " << DISTS[start.id][p.id] << "  " << p << "\n";
        }
    }
}

int MIN_STEPS = INF;
void backtracking(std::vector<Portal> const& portals, int steps, int portalId, int depth) {
    if (depth > 25) {
        return;  // input requires a depth of 25
    }
    if (steps > MIN_STEPS) {
        return;
    }

    if (portals[portalId].name == "ZZ" && depth == 0) {
        if (steps < MIN_STEPS) {
            std::cerr << "New best: " << steps << "\n";
            MIN_STEPS = steps;
        }
        return;
    }

    int hash = 0;
    hash = portalId + depth * 100;
    if (hashTable.contains(hash)) {
        int hashSteps = hashTable[hash];
        if (steps >= hashSteps) {
            return;
        } else {
            hashTable[hash] = steps;
            // std::cerr << "new path hash " << steps << " " << hashSteps << " ( " << MIN_STEPS << "\n";
        }
    } else {
        hashTable[hash] = steps;
    }

    for (auto& portal : portals) {
        if (portal.id != portalId && DISTS[portalId][portal.id] != INF) {
            backtracking(portals, steps + DISTS[portalId][portal.id], portal.id, depth);
        }
    }

    Portal const& portal = portals[portalId];
    if (portal.exitId != -1) {
        if (depth == 0 && portal.type == OUTER) {
            // cannot leave further
            return;
        }

        int deltaDepth = portal.type == INNER ? +1 : -1;

        backtracking(portals, steps + 1, portal.exitId, depth + deltaDepth);
    }
}

int main() {
    std::string str;
    matrix<char> grid;
    while (getline(std::cin, str)) {
        grid.emplace_back();
        std::vector<char>& last = grid.back();
        // std::cerr << str << "\n";
        for (auto& c : str) {
            last.push_back(c);
        }
    }

    WIDTH = grid[0].size();
    HEIGHT = grid.size();
    std::cerr << WIDTH << ", " << HEIGHT << "\n";
    matrix<char> newGrid (WIDTH, std::vector<char>(HEIGHT));

    for (int x = 0; x < WIDTH; ++x) {
        for (int y = 0; y < HEIGHT; ++y) {
            newGrid[x][y] = grid[y][x];
        }
    }

    grid = newGrid;

    std::vector<Portal> portals;


    for (int x = 0; x < WIDTH; ++x) {
        for (int y = 0; y < HEIGHT; ++y) {
            if (grid[x][y] == '.') {
                for (auto& dir : directions) {
                    Point neigh = Point(x, y) + dir;
                    char c = grid[neigh.x][neigh.y];
                    if ('A' <= c && c <= 'Z') {
                        // new portal
                        Point neigh2 = neigh + dir;
                        char c2 = grid[neigh2.x][neigh2.y];
                        std::string name = "";
                        if (dir == Point(-1, 0) || dir == Point(0, -1)) {
                            name += c2; name += c;
                        } else {
                            name += c; name += c2;
                        }

                        Portal portal;
                        portal.name = name;
                        portal.type = INNER;
                        if (x == 2 || x == WIDTH - 3 || y == 2 || y == HEIGHT - 3) {
                            portal.type = OUTER;
                        }
                        portal.entry = Point(x, y);
                        portal.id = portals.size();
                        portals.push_back(portal);
                    }
                }
            }
        }
    }

    Portal start, end;
    for (auto& portal1 : portals) {
        if (portal1.name == "AA") {
            start = portal1;
        }
        if (portal1.name == "ZZ") {
            end = portal1;
        }

        for (auto& portal2 : portals) {
            if (portal1.id == portal2.id) {
                continue;
            }

            if (portal1.name == portal2.name) {
                portal1.exit = portal2.entry;
                portal1.exitId = portal2.id;

                portal2.exit = portal1.entry;
                portal2.exitId = portal1.id;

                break;
            }
        }
    }

    for (auto& p : portals) {
        std::cerr << p << "\n";
        bfs(grid, portals, p);
    }



    std::cerr << "\n" << start << "\n" << end << "\n";

    for (int y = 0; y < HEIGHT; ++y) {
        for (int x = 0; x < WIDTH; ++x) {
            std::cerr << grid[x][y];
        }
        std::cerr << "\n";
    }

    backtracking(portals, 0, start.id, 0);
    // bfs(grid, portals, start, end);
}