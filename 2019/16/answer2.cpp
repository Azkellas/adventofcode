#include <iostream>
#include <cstring>
#include <vector>
int main() {
    std::string str;
    std::cin >> str;
    int size = str.size();
    int MILLE = 10'000;
    int realSize = MILLE*size;
    std::vector<int8_t> arr (realSize);
    int patternSize = 4*realSize;
    std::cerr << sizeof(arr) << "\n";
    for (int i = 0; i < size; ++i) {
        for (int k = 0; k < MILLE; ++k) {
            arr[k*size + i] = str[i] - '0';
        }
    }

    int offset = 0;
    for (int i = 0; i < 7; ++i) {
        offset = 10*offset + arr[i];
    }
    std::cerr << "offset " << offset <<"\n";
    std::cerr << (offset > realSize/2) << "\n";
    // std::cerr << "step " << 0 << ": ";
    // for (auto& a : arr) {
    //     std::cerr << (int)a;
    // }
    // std::cerr << "\n";
    for (int step = 0; step < 100; ++step) {
        std::cerr << "step " << step << "\n";
        for (int i = realSize - 2; i >= offset; --i) {
            arr[i] = (arr[i] + arr[i+1]) % 10;
        }
    }

    for (int i = offset; i < offset + 8; ++i) {
        std::cerr << (int)arr[i];
    }
    std::cerr << "\n";
}