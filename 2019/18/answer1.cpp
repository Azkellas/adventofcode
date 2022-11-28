#include <iostream>
#include <cstring>
#include <vector>
#include <queue>
#include <unordered_map>

template <typename T>
using matrix = std::vector<std::vector<T>>;

int WIDTH, HEIGHT;


int N_KEYS = 0;
std::vector<int> validIds;
int DISTS[100][100];
int INF = 100'000;
int MIN_STEPS = INF;

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

bool operator==(Point const& p, Point const& q) {
    return p.x == q.x && p.y == q.y;
}

bool operator!=(Point const& p, Point const& q) {
    return !(p == q);
}

std::ostream& operator<<(std::ostream& os, Point const& p) {
    os << "(" << p.x << ", " << p.y << ")";
    return os;
}


Point operator+(Point const& p, Point const& q) {
    return {p.x + q.x, p.y + q.y};
}


bool isDoor(char c) {
    return c >= 'A' && c <= 'Z';
}

bool isKey(char c) {
    return c >= 'a' && c <= 'z';
}

Point directions[4] = {Point(-1, 0), Point(0, -1), Point(1, 0), Point(0, 1)};

int keyId(char c) {
    return c - 'a';
}

int doorId(char c) {
    return c - 'A' + 50;
}

int starter() {
    return 99;
}

int getId(char c) {
    if (isKey(c)) {
        return keyId(c);
    } else if (isDoor(c)) {
        return doorId(c);
    } else if (c == '@') {
        return starter();
    } else {
        std::cerr << "invalid id " << c << "\n";
        return -1;
    }
}

char getChar(int id) {
    if (id < 50) {
        return id + 'a';
    } else if (id != 99) {
        return id - 50 + 'A';
    } else {
        return '@';
    }
}

void bfs(matrix<char> const& grid, Point const& location) {
    int locId = getId(grid[location.x][location.y]);
    int depths[WIDTH][HEIGHT];
    std::memset(depths, INF, sizeof(depths[0][0]) * WIDTH * HEIGHT);
    for (int x = 0; x < WIDTH; ++x) {
        for (int y = 0; y < HEIGHT; ++y) {
            depths[x][y] = INF;
        }
    }

    std::queue<Point> q;
    q.push(location);
    depths[location.x][location.y] = 0;
    while (!q.empty()) {
        Point loc = q.front();
        // std::cerr << loc << "\n";
        q.pop();
        for (auto& dir : directions) {
            Point neigh = loc + dir;
            if (neigh.isInside() && grid[neigh.x][neigh.y] != '#') {
                if (depths[neigh.x][neigh.y] > depths[loc.x][loc.y] + 1) {
                    depths[neigh.x][neigh.y] = depths[loc.x][loc.y] + 1;

                    if (grid[neigh.x][neigh.y] == '.') {
                        // empty cell
                        q.push(neigh);
                    }

                    // doors / keys / starter are dead ends
                }
            }
        }
    }


    for (int x = 0; x < WIDTH; ++x) {
        for (int y = 0; y < HEIGHT; ++y) {
            if (Point(x, y) != location && depths[x][y] != INF && grid[x][y] != '#' && grid[x][y] != '.') {
                int dist = depths[x][y];
                int id = getId(grid[x][y]);
                DISTS[locId][id] = dist;
            }
        }
    }
}


void backtracking(int steps, int locId, std::vector<bool> const& open, int keysToOpen) {
    if (steps > MIN_STEPS) {
        return;
    }

    if (keysToOpen == 0) {
        if (steps < MIN_STEPS) {
            std::cerr << "New best: " << steps << "\n";
            MIN_STEPS = steps;
        }
        return;
    }

    int hash = 0;
    hash ^= std::hash<std::vector<bool>>{}(open);
    hash ^= locId;
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
    // for (int i = 0; i < N_KEYS - keysToOpen; ++i) {
    //     std::cerr << "| ";
    // }
    // std::cerr << getChar(locId) << " (" << steps << ")\n";

    std::vector<int> dists(100, INF);
    dists[locId] = 0;
    std::queue<int> q;
    q.push(locId);
    while (!q.empty()) {
        int id = q.front();
        q.pop();
        // std::cerr << "           seeing " << getChar(id) << "\n";
        for (auto& neighId : validIds) {
            if (dists[neighId] > dists[id] + DISTS[id][neighId]) {
                // std::cerr << "               neigh: " << getChar(neighId);
                dists[neighId] = dists[id] + DISTS[id][neighId];

                if (isDoor(getChar(neighId)) && open[neighId - 50]) {
                    // std::cerr << "  (pushed)";
                    q.push(neighId);
                }

                if (getChar(neighId) == '@') {
                    // std::cerr << "  (pushed)";
                    q.push(neighId);
                }

                if (isKey(getChar(neighId)) && open[neighId]) {
                    // std::cerr << "  (pushed)";
                    q.push(neighId);
                }
                // std::cerr << "\n";
            }
        }
    }

    auto cmp = [&](int id1, int id2) { return dists[id1] >= dists[id2]; };
    std::priority_queue<int, std::vector<int>, decltype(cmp)> p(cmp);

    for (int keyId = 0; keyId < N_KEYS; ++keyId) {
        if (!open[keyId] && dists[keyId] != INF) {
            p.push(keyId);
        }
    }

    while (!p.empty()) {
        int keyId = p.top();
        p.pop();
        std::vector<bool> newOpen = open;
        newOpen[keyId] = true;
        backtracking(steps + dists[keyId], keyId, newOpen, keysToOpen - 1);
    }
}


int linked(int key, int door) {
    return door - 50 == key;
}

int main() {
    for (int i = 0; i < 100; ++i) {
        for (int j = 0; j < 100; ++j) {
            DISTS[i][j] = INF;
        }
    }

    std::string str;
    matrix<char> grid;
    Point location;
    while (getline(std::cin, str)) {
        grid.emplace_back();
        std::vector<char>& last = grid.back();
        // std::cerr << str << "\n";
        for (auto& c : str) {
            if (c == '@') {
                location = Point(last.size(), grid.size() -1);
            }

            last.push_back(c);
        }
    }

    WIDTH = grid[0].size();
    HEIGHT = grid.size();
    std::cerr << WIDTH << ", " << HEIGHT << "\n";
    std::cerr << location << "\n";
    matrix<char> newGrid (WIDTH, std::vector<char>(HEIGHT));
    for (int x = 0; x < WIDTH; ++x) {
        for (int y = 0; y < HEIGHT; ++y) {
            newGrid[x][y] = grid[y][x];
        }
    }

    grid = newGrid;

    for (int y = 0; y < HEIGHT; ++y) {
        for (int x = 0; x < WIDTH; ++x) {
            std::cerr << grid[x][y];
        }
        std::cerr << "\n";
    }

    N_KEYS = 0;
    for (int x = 0; x < WIDTH; ++x) {
        for (int y = 0; y < HEIGHT; ++y) {
            if (grid[x][y] != '#' && grid[x][y] != '.') {
                if (isKey(grid[x][y])) {
                    N_KEYS++;
                }
                bfs(grid, Point(x, y));
                validIds.push_back(getId(grid[x][y]));
            }
        }
    }

    for (auto& id1 : validIds) {
        for (auto& id2 : validIds) {
            if (DISTS[id1][id2] != INF) {
                std::cerr << getChar(id1) << " -> " << getChar(id2) << " = " << DISTS[id1][id2] << "\n";
            }
        }
    }

    std::vector<bool> open (N_KEYS, false);
    backtracking(0, starter(), open, N_KEYS);
    std::cout << "Best score: " << MIN_STEPS << "\n";
}