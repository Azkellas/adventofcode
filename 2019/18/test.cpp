#include <iostream>
#include <vector>

using namespace std;
int main() {
    vector<bool> v;
    v.push_back(false);
    v.push_back(true);
    v.push_back(false);
    v.push_back(false);
    v.push_back(true);

    std::cerr << std::hash<vector<bool>>{}(v) << "\n";
    std::cerr << std::hash<int>{}(12) << "\n";
    std::cerr << (12 << 20) << " "  << std::hash<int>{}(12 << 20) << "\n";

    auto h = std::hash<vector<bool>>{}(v) ^ std::hash<int>{}(12) ^ std::hash<int>{}(12 << 30);

    std::cerr << h << "\n";
}