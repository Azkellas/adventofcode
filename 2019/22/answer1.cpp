#include <iostream>
#include <array>
#include <cstring>
#include <vector>

int INCREMENT = 0;
int NEW_STACK = 1;
int CUT = 2;

constexpr int N_CARDS = 10'007;

using Deck = std::array<int, N_CARDS>;
Deck deck1;
Deck deck2;

bool fromDeck1 = true;

class Operation {
 public:
    int type;
    int incr;
    int cut;
};

std::vector<Operation> operations;

void newStack() {
    Deck* from;
    Deck* to;
    if (fromDeck1) {
        from = &deck1;
        to = &deck2;
    } else {
        from = &deck2;
        to = &deck1;
    }

    for (int i = 0; i < N_CARDS; ++i) {
        to->at(i) = from->at(N_CARDS - 1 - i);
    }

    fromDeck1 = !fromDeck1;
}

void cut (int cut) {
    if (cut < 0) {
        cut += N_CARDS;
    }
    Deck* from;
    Deck* to;
    if (fromDeck1) {
        from = &deck1;
        to = &deck2;
    } else {
        from = &deck2;
        to = &deck1;
    }

    for (int i = cut; i < N_CARDS; ++i) {
        to->at(i - cut) = from->at(i);
    }
    for (int i = 0; i < cut; ++i) {
        to->at(i + N_CARDS - cut) = from->at(i);
    }

    fromDeck1 = !fromDeck1;
}

void increment(int incr) {
    Deck* from;
    Deck* to;
    if (fromDeck1) {
        from = &deck1;
        to = &deck2;
    } else {
        from = &deck2;
        to = &deck1;
    }

    int idx = 0;
    int seen = 0;
    for (int seen = 0; seen < N_CARDS; ++seen) {
        to->at(idx) = from->at(seen);
        idx += incr;
        if (idx >= N_CARDS) {
            idx -= N_CARDS;
        }
    }

    fromDeck1 = !fromDeck1;
}

int main() {
    // init
    for (int i = 0; i < N_CARDS; ++i) {
        deck1[i] = i;
    }

    std::string line;
    while (getline(std::cin, line)) {
        std::size_t found;
        std::string toFind;
        bool done = false;
    
        toFind = "cut";
        found = line.find(toFind);
        if (found != std::string::npos) {
            std::string nbr = line.substr(found + toFind.size() + 1, std::string::npos);
            int cut = std::stoi(nbr);
            std::cerr << "CUT " << cut << "\n";
            Operation ope;
            ope.type = CUT;
            ope.cut = cut;
            operations.push_back(ope);
            done = true;
        }

        toFind = "deal with increment";
        found = line.find(toFind);
        if (found != std::string::npos) {
            std::string nbr = line.substr(found + toFind.size() + 1, std::string::npos);
            int incr = std::stoi(nbr);
            std::cerr << "INCR " << incr << "\n";
            Operation ope;
            ope.type = INCREMENT;
            ope.incr = incr;
            operations.push_back(ope);
            done = true;
        }

        toFind = "deal into new stack";
        found = line.find(toFind);
        if (found != std::string::npos) {
            std::cerr << "NEW STACK" << "\n";
            Operation ope;
            ope.type = NEW_STACK;
            operations.push_back(ope);
            done = true;
        }

        if (!done) {
            std::cerr << "PB " << line << "\n";
        }
    }


    for (auto& ope : operations) {
        if (ope.type == CUT) {
            cut(ope.cut);
        }
        if (ope.type == INCREMENT) {
            increment(ope.incr);
        }
        if (ope.type == NEW_STACK) {
            newStack();
        }
    }

    Deck* from;
    if (fromDeck1) {
        from = &deck1;
    } else {
        from = &deck2;
    }

    for (int i = 0; i < N_CARDS; ++i) {
        if (from->at(i) == 2019) {
            std::cout << "2019 at " << i << "\n";
        }
    }
}