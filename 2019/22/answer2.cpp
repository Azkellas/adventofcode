#include <iostream>
#include <array>
#include <cstring>
#include <vector>

constexpr int INCREMENT = 0;
constexpr int NEW_STACK = 1;
constexpr int CUT = 2;

constexpr long long N_CARDS = 119315717514047;
// constexpr long long N_CARDS = 10'007;

class Operation {
 public:
    int type;
    long long incr;
    long long cut;
};

std::vector<Operation> operations;


long long exponent(long long base, long long exp, long long modulus) {
    long long res = 1;
    base = base % modulus;
    while (exp > 0) {
        if (exp % 2 == 1) {
            res = (res * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    return res;
}


int main() {

    long long base = 81'904'140'893'450;
    long long exp = 2'076'661;
    std::cerr << exponent(base, exp, N_CARDS) << "\n";;
    long long res = 1;
    for (int i = 0; i < exp; ++i) {
        res = (res * base) % N_CARDS;
    }
    std::cerr << res << "\n";
    return 0;
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
            // std::cerr << "CUT " << cut << "\n";
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
            // std::cerr << "INCR " << incr << "\n";
            Operation ope;
            ope.type = INCREMENT;
            ope.incr = incr;
            operations.push_back(ope);
            done = true;
        }

        toFind = "deal into new stack";
        found = line.find(toFind);
        if (found != std::string::npos) {
            // std::cerr << "NEW STACK" << "\n";
            Operation ope;
            ope.type = NEW_STACK;
            operations.push_back(ope);
            done = true;
        }

        if (!done) {
            std::cerr << "PB " << line << "\n";
        }
    }

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
            newOperations.push_back(operations[i]);
        }
        operations = newOperations;
    }

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

    long long position = 2020;
    std::cerr << N_CARDS << "\n";
    long long TOTAL_STEPS = 101741582076661;
    101'741'582'076'661;  // steps
    119'315'717'514'047;  // cards
     30'251'038'085'100; // off
     93'413'888'090'395; // inc
         38'000'000'000;
     81'904'140'893'450;  // incr
     39'308'085'444'041;  // cut
    long long step = 0;
    long long lastCerr = 0;
    long long incr = operations[0].incr;
    long long cuta = operations[2].cut;

    // while (step < N_CARDS - TOTAL_STEPS) {
    for (long long step = 0; step < N_CARDS - TOTAL_STEPS; ++step) {
        if (step == lastCerr) {
            std::cerr << step << "\n";
            lastCerr += 1'000'000'000;
        }
    
        position = (- (1 + incr * position + cuta)) % N_CARDS;
    }

    // pos = - (1 + incr*pos + cuta)
    // pos = A*pos + B
    //      A = - incr
    //      B = - 1 - cuta
    // pos = A*pos + B
    // pos = A*(A*pos+B)+B = A2*pos+ AB+B
    // pos = A*pos + B = A3*pos + A2*B + AB + B
    // pos = pow(A, steps)* 2020 + sum(0, steps, pow(A,i)*B)

    std::cerr << "end position = " << position << "\n";
}
