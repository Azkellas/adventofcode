#include <iostream>
#include <array>
#include <cstring>
#include <vector>

constexpr int INCREMENT = 0;
constexpr int NEW_STACK = 1;
constexpr int CUT = 2;

constexpr int N_CARDS = 10'007;
// constexpr int N_CARDS = 23;

using Deck = std::array<int, N_CARDS>;
Deck deck1;
Deck deck2;

bool fromDeck1 = true;

class Operation {
 public:
    int type;
    long long incr;
    long long cut;
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

void cut (long long cut) {
    cut %= N_CARDS;
    while (cut < 0) {
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

void increment(long long incr) {
    Deck* from;
    Deck* to;
    if (fromDeck1) {
        from = &deck1;
        to = &deck2;
    } else {
        from = &deck2;
        to = &deck1;
    }

    long long idx = 0;
    long long seen = 0;
    for (int seen = 0; seen < N_CARDS; ++seen) {
        to->at(idx) = from->at(seen);
        idx += incr;
        idx %= N_CARDS;
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

    // for (int i = 0; i < N_CARDS; ++i) {
    //     std::cerr << deck1[i] << " ";
    // }
    // std::cerr << "\n";

    // newStack();
    // increment(3);
    // Deck* fromD = (fromDeck1 ? &deck1 : &deck2);
    // for (int i = 0; i < N_CARDS; ++i) {
    //     std::cerr << fromD->at(i) << " ";
    // }
    // std::cerr << "\n";

    // for (int i = 0; i < N_CARDS; ++i) {
    //     deck1[i] = i;
    // }
    // fromDeck1 = true;

    // increment(3);
    // newStack();
    // cut(2);
    // fromD = (fromDeck1 ? &deck1 : &deck2);
    // for (int i = 0; i < N_CARDS; ++i) {
    //     std::cerr << fromD->at(i) << " ";
    // }
    // std::cerr << "\n";
    // return 0;

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
            Operation ope;
            ope.type = INCREMENT;
            ope.incr = incr;
            operations.push_back(ope);
            done = true;
        }

        toFind = "deal into new stack";
        found = line.find(toFind);
        if (found != std::string::npos) {
            Operation ope;
            ope.type = NEW_STACK;
            operations.push_back(ope);
            done = true;
        }

        if (!done) {
            std::cerr << "PB " << line << "\n";
        }
    }

    int firstSize = operations.size();

    bool changed = true;
    while (changed) {
        changed = false;
        std::vector<Operation> newOperations;
        int i = 0;
        while (i < operations.size() - 1) {
            Operation const& op1 = operations[i];
            Operation const& op2 = operations[i+1];
            if (i < operations.size() - 2) {
                Operation const& op3 = operations[i+2];
                if (op1.type == NEW_STACK && op3.type == NEW_STACK) {
                    if (op2.type == INCREMENT) {
                        Operation newOp;
                        newOp.type = CUT;
                        newOp.cut = -op2.incr + 1;
                        newOperations.push_back(op2);
                        newOperations.push_back(newOp);
                        i += 3;
                        changed = true;
                        continue;
                    }
                    if (op2.type == CUT) {
                        Operation newOp;
                        newOp.type = CUT;
                        newOp.cut = -op2.cut;
                        newOperations.push_back(newOp);
                        i += 3;
                        changed = true;
                        continue;
                    }
                }
            }

            if (op1.type == INCREMENT && op2.type == INCREMENT) {
                Operation newOp;
                newOp.type = INCREMENT;
                newOp.incr = (op1.incr * op2.incr) % N_CARDS;
                if (newOp.incr < 0) {
                    std::cerr << newOp.incr << "\n";
                }
                newOperations.push_back(newOp);
                i += 2;
                changed = true;
                continue;
            }


            if (op1.type == CUT && op2.type == CUT) {
                Operation newOp;
                newOp.type = CUT;
                newOp.cut = (op1.cut + op2.cut) % N_CARDS;
                newOperations.push_back(newOp);
                i += 2;
                changed = true;
                continue;
            }

            if (op1.type == CUT && op2.type == INCREMENT) {
                Operation newOp;
                newOp.type = CUT;
                newOp.cut = (op1.cut * op2.incr) % N_CARDS;
                newOperations.push_back(op2);
                newOperations.push_back(newOp);
                i += 2;
                changed = true;
                continue;
            }

            if (op1.type == CUT && op2.type == NEW_STACK) {
                Operation newOp;
                newOp.type = CUT;
                newOp.cut = -op1.cut;
                newOperations.push_back(op2);
                newOperations.push_back(newOp);
                i += 2;
                changed = true;
                continue;
            }

            if (op1.type == NEW_STACK && op2.type == INCREMENT) {
                Operation newOp;
                newOp.type = CUT;
                newOp.cut = op2.incr - 1;
                newOperations.push_back(op2);
                newOperations.push_back(op1);
                newOperations.push_back(newOp);
                i += 2;
                changed = true;
                continue;
            }

            newOperations.push_back(op1);
            i += 1;
        }

        if (i == operations.size() - 1) {
            if (operations[i].type == INCREMENT && operations[i].incr < 0) {
                std::cerr << "h\n";
            }
            newOperations.push_back(operations[i]);
        }
        operations = newOperations;
    }


    std::cerr << "sizes: " << firstSize << " => " << operations.size() << "\n";
    for (auto& op : operations) {
        switch (op.type) {
            case CUT:
                std::cerr << "CUT " << op.cut << "\n";
                break;
            case INCREMENT:
                std::cerr << "INCR " << op.incr << "\n";
                break;
            case NEW_STACK:
                std::cerr << "STACK" << "\n";
                break;

        }
    }

    // long long pos1 = 2019;
    // long long pos2 = 2019;
    long long incr = operations[0].incr;
    long long cuta = operations[2].cut;

    // position0 = 2019
    // position1 = incr(2019) = incr * position0
    // position2 = stack(position1) = - (1 + position1)
    // position3 = cut(position2) = position2 - ope.cut
    long long finalPos = (- (1 + incr * (2019) + cuta)) % N_CARDS;
    if (finalPos < 0) {
        finalPos += N_CARDS;
    }
    std::cerr << "finale pos "  << finalPos << "\n";
    // return 0;

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