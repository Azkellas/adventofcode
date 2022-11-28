#include <iostream>
#include <cstring>
#include <vector>
#include <queue>
#include <unordered_map>

template <typename T>
using matrix = std::vector<std::vector<T>>;

int WIDTH, HEIGHT;

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


Point directions[4] = {Point(-1, 0), Point(0, -1), Point(1, 0), Point(0, 1)};


int INF = 1'000'000;
void bfs(matrix<char> const& grid, matrix<Point> portals, Point const& start, Point const& end) {
    int depths[WIDTH][HEIGHT];
    std::memset(depths, INF, sizeof(depths[0][0]) * WIDTH * HEIGHT);
    for (int x = 0; x < WIDTH; ++x) {
        for (int y = 0; y < HEIGHT; ++y) {
            depths[x][y] = INF;
        }
    }

    std::queue<Point> q;
    q.push(start);
    depths[start.x][start.y] = 0;

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

        // check teleporter
        if (portals[loc.x][loc.y].isInside()) {
            Point port = portals[loc.x][loc.y];
            if (depths[port.x][port.y] > depths[loc.x][loc.y] + 1) {
                depths[port.x][port.y] = depths[loc.x][loc.y] + 1;
                q.push(port);
            }
        }
    }

    std::cerr << "dist = " << depths[end.x][end.y] << "\n";
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

    matrix<Point> portals (WIDTH, std::vector<Point>(HEIGHT));

    for (int x = 0; x < WIDTH; ++x) {
        for (int y = 0; y < HEIGHT; ++y) {
            newGrid[x][y] = grid[y][x];
        }
    }

    grid = newGrid;

    std::unordered_map<std::string, Point> savePortals;
    Point start;
    Point end;

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
                        if (name == "AA") {
                            start = Point(x, y);
                        }
                        if (name == "ZZ") {
                            end = Point(x, y);
                        }
                        if (savePortals.contains(name)) {
                            Point exit = savePortals[name];
                            portals[exit.x][exit.y] = Point(x, y);
                            portals[x][y] = exit;
                        } else {
                            savePortals[name] = Point(x, y);
                        }
                    }
                }
            }
        }
    }


    for (int y = 0; y < HEIGHT; ++y) {
        for (int x = 0; x < WIDTH; ++x) {
            if (portals[x][y] != Point(-1, -1)) {
                std::cerr << Point(x, y) << ": " << portals[x][y] << "\n";;
            }
        }
    }

    std::cerr << start << " -> " << end << "\n";

    for (int y = 0; y < HEIGHT; ++y) {
        for (int x = 0; x < WIDTH; ++x) {
            std::cerr << grid[x][y];
        }
        std::cerr << "\n";
    }

    // for (int x = 0; x < WIDTH; ++x) {
    //     for (int y = 0; y < HEIGHT; ++y) {
    //         if (grid[x][y] != '#' && grid[x][y] != '.') {
    //             if (isKey(grid[x][y])) {
    //                 N_KEYS++;
    //             }
    //             bfs(grid, Point(x, y));
    //             validIds.push_back(getId(grid[x][y]));
    //         }
    //     }
    // }

    bfs(grid, portals, start, end);
}