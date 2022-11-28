#include <iostream>
#include <vector>
#include <unordered_map>
#include <sstream>
#include <set>
#include <array>
#include <cmath>

enum Rotation {
    rot0 = 0,
    rot90,
    rot180,
    rot270
};


enum Side {
    N = 0,
    E,
    S,
    W
};

enum Flip {
    flipN = 0,
    flipH,
    flipV
};

class Operation
{
public:
    Rotation rota {rot0};
    Flip flip {flipN};
};

class Line
{
public:
    char arr[10];
    void reverse() {
        // std::cerr << "reverse ";
        // for (int i = 0; i < 10; ++i) std::cerr << arr[i];
        // std::cerr << ' ';
        for (int i = 0; i < 5; ++i)
            std::swap(arr[i], arr[9-i]);
        // for (int i = 0; i < 10; ++i) std::cerr << arr[i];
        // std::cerr << '\n';
    }
};

class Grid
{
public:
    char arr[10][10];
};


class Tile
{
public:
    int id;
    Grid grid;
};

std::ostream& operator<<(std::ostream& os, Tile const& p_tile)
{
    os << "Tile " << p_tile.id << '\n';
    for (int y = 0; y < 10; ++y) {
        for (int x = 0; x < 10; ++x)
            os << p_tile.grid.arr[y][x];
        os << '\n';
    }
    return os;
}

class Key
{
public:
    int tile {-1};
    Operation op;
};

std::ostream& operator<<(std::ostream& os, Key const& p_key)
{
    os  << (int)p_key.tile << '('
        << (int)p_key.op.rota * 90 << ','
        << (p_key.op.flip == flipN ? 'N'
            : p_key.op.flip == flipV ? 'V' : 'H')
        << ')';
    return os;
}

bool operator==(Key const& p_lhs, Key const& p_rhs) {
    return p_lhs.tile == p_rhs.tile && p_lhs.op.rota == p_rhs.op.rota && p_lhs.op.flip == p_rhs.op.flip;
}
bool operator!=(Key const& p_lhs, Key const& p_rhs) {
    return !(p_lhs == p_rhs);
}
bool operator<(Key const& p_lhs, Key const& p_rhs) {
    return p_lhs.tile <  p_rhs.tile ||
           p_lhs.tile == p_rhs.tile && p_lhs.op.rota <  p_rhs.op.rota ||
           p_lhs.tile == p_rhs.tile && p_lhs.op.rota == p_rhs.op.rota && p_lhs.op.flip < p_rhs.op.flip;
}

namespace std {
    template <>
    struct hash<Key> {
        std::size_t operator()(const Key& k) const {
            return k.tile << 4 + k.op.rota << 2 + k.op.flip;
        }
    };
}

Line get_side(Grid const p_grid, Operation const & p_op, Side p_side)
{
    Line res;

    p_side = (Side)((int)p_side - (int)p_op.rota);
    if (p_side < 0) p_side = (Side)((int)p_side + 4);

    if (p_op.flip == flipV && (p_side == N || p_side == S))
        p_side = (Side)((int)p_side + 2);
    if (p_op.flip == flipH && (p_side == W || p_side == E))
        p_side = (Side)((int)p_side + 2);
    if (p_side >= 4) p_side = (Side)((int)p_side - 4);

    switch (p_side) {
    case N:
        for (int i = 0; i < 10; ++i) 
            res.arr[i] = p_grid.arr[0][i];
        break;
    case E:
        for (int i = 0; i < 10; ++i) 
            res.arr[i] = p_grid.arr[i][9];
        break;
    case S:
        for (int i = 0; i < 10; ++i) 
            res.arr[i] = p_grid.arr[9][9-i];
        break;
    case W:
        for (int i = 0; i < 10; ++i) 
            res.arr[i] = p_grid.arr[9-i][0];
        break;
    }

    if (p_op.flip != flipN) {
        res.reverse();
    }

    return res;
}

std::vector<Key> keys;

std::unordered_map<Key, std::array<std::unordered_map<Key, bool>, 4>> links;
std::unordered_map<int, Tile> tiles;
std::set<Key> all_keys;

int grid_size;
int line_size;

void compute_all_keys()
{
    Key key;
    for (auto& [id, tile] : tiles) {
        key.tile = id;
        for (int rot = 0; rot < 4; ++rot) {
            for (int flip = 0; flip < 3; ++flip) {
                key.op.rota = (Rotation)rot;
                key.op.flip = (Flip)flip;

                all_keys.insert(key);
            }
        }
    }
}

void compute_all_links()
{
    int total_valid = 0;
    Key key2;
    for (auto& key1 : all_keys) {
        for (int side = 0; side < 4; ++side) {
            Line line1 = get_side(tiles[key1.tile].grid, key1.op, (Side)side);
            for (auto& key2 : all_keys) {
                Line line2 = get_side(tiles[key2.tile].grid, key2.op, (Side)(((int)side + 2) % 4));
                line2.reverse();
                bool valid = true;
                for (int i = 0; i < 10; ++i) {
                    if (line1.arr[i] != line2.arr[i]) {
                        valid = false;
                        break;
                    }
                }

                if (key1.tile == key2.tile) valid = false;

                links[key1][side][key2] = valid;

                if (valid) {
                    // std::cerr << side << " " << key1 << " < - > " << key2 << "\n";

                    // for (int i = 0; i < 10; ++i) std::cerr << line1.arr[i];
                    // std::cerr << "\n";
                    // for (int i = 0; i < 10; ++i) std::cerr << line2.arr[i];
                    // std::cerr << "\n";
                    ++total_valid;
                }
            }
        }
    }
    std::cerr << "total valid " << total_valid << "\n";
}

int MAX_DEPTH = 0;
void rec(int p_idx)
{
    if (p_idx == grid_size) {
        std::cerr << "found solution!\n";
        for (int i = 0; i < p_idx; ++i)
            std::cerr << keys[i] << "\n";
        unsigned long long total = 1;
        int s = tiles.size();
        int ss = std::sqrt(s);
        total *= keys[0].tile;
        total *= keys[line_size - 1].tile;
        total *= keys[grid_size - line_size].tile;
        total *= keys[grid_size - 1].tile;
        std::cerr << "res: " << total << "\n";
        *(int*)0 = 0;
    }

    int left_idx = p_idx - 1;
    if (p_idx % line_size == 0) left_idx = -1;

    int abve_idx = p_idx - line_size;

    bool found = false;
    bool valid;
    for (auto& key : all_keys) {
        valid = true;
        valid &= left_idx < 0 || links[key][W][keys[left_idx]];
        valid &= abve_idx < 0 || links[key][N][keys[abve_idx]];
        if (valid) {
            for (int i = 0; i < p_idx; ++i) {
                if (keys[i].tile == key.tile) {
                    valid = false;
                    break;
                }
            }
        }
        if (valid) {
            keys[p_idx] = key;
            rec(p_idx + 1);
            found = true;
        }
    }

    if (!found && p_idx > MAX_DEPTH) {
        MAX_DEPTH = p_idx;
        std::cerr << "new max dpth " << MAX_DEPTH << '\n';
    }
} 


int main()
{
    Tile tile;
    int next_i = 0;

    std::string str;
    while (std::getline(std::cin, str)) {
        if (str == "") {
            tiles[tile.id] = tile;
            next_i = 0;
        }

        else if (str[0] == 'T') {
            std::istringstream ss (str);
            ss >> str >> tile.id;
        }

        else {
            for (int i = 0; i < 10; ++i) {
                tile.grid.arr[next_i][i] = str[i];
            }
            ++next_i;
        }
    }

    // for (auto& [id, tile] : tiles) {
    //     std::cerr << tile << '\n';
    // }

    grid_size = tiles.size();
    line_size = std::sqrt(grid_size);
    keys = std::vector<Key>(grid_size);

    compute_all_keys();
    compute_all_links();

    rec(0);

    // std::cerr << "links\n";
    // for (auto& [key, set] : links) {
    //     std::cerr << (int)key.tile << " " << (int)key.op.rota << " " << (int)key.op.flip << ":\n";
    //     for (auto& val : set) {
    //         std::cerr << "  " << (int)val.tile << " " << (int)val.op.rota << " " << (int)val.op.flip;
    //     }
    //     std::cerr << "\n";
    // }
}
