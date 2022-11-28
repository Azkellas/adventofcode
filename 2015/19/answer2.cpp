#include <iostream>
#include <cstring>
#include <vector>
#include <sstream>
#include <set>
#include <unordered_map>
#include <cassert>

class Rule
{
public:
    std::string from;
    std::string to;

    int from_size;
    int to_size;
    int delta;

    std::unordered_map<std::string, int> counts;
    std::unordered_map<std::string, int> permanent_counts;

    inline void setup() {
        from_size = from.size();
        to_size   = to.size();
        delta     = to_size - from_size;
    }
};

std::vector<Rule> rules;
int MAX_LENGTH = 0;
std::set<uint64_t> seen;
std::string target;

std::vector<std::vector<uint64_t>> hashes;

std::unordered_map<std::string, int> counts;
std::unordered_map<std::string, int> permanent_counts;

std::set<std::string> permanent_keys = {"Th", "Ar", "Rn", "Si", "Y", "CRn"};
std::set<std::string> keys;

class Formula
{
public:
    std::vector<char> array;
    int length;

    std::unordered_map<std::string, int> counts;
    std::unordered_map<std::string, int> permanent_counts;

    void print() const {
        std::cerr << "formula: ";
        for (int i = 0; i < length; ++i)
            std::cerr << array[i];
        std::cerr << '\n';
    }

    uint64_t get_hash() const {
        uint64_t hash = 0;
        for (int i = 0; i < length; ++i) {
            hash ^= hashes[i][array[i] - 'A'];
        }
        return hash;
    }

    void setup() {
        array = std::vector<char> (MAX_LENGTH);
        array[0] = 'e';
        length = 1;

        for (auto const & key : permanent_counts) {
            this->permanent_counts[key.first] = 0;
        }
        for (auto const & key : counts) {
            this->counts[key.first] = 0;
        }

        this->counts["e"] = 1;
    }

    void print_keys() const {
        print();
        for (auto& key : keys) {
            if (this->counts.at(key) != 0) {
                std::cerr << "  | " << key << ": " << this->counts.at(key) << "\n";
            }
        }
        std::cerr << '\n';
    }
};

void replace(Formula * p_formula, Rule const & p_rule, int const p_idx, bool reverse = false);


void __same(Formula form1, Formula form2, Rule const & rule, int idx)
{
        std::cerr << "issue !!\n";
        form1.print_keys();
        form2.print_keys();
        std::cout << "for rule: " << rule.from << " => " << rule.to << "\n"; 

        std::cerr << "middle formula\n";
        replace(&form1, rule, idx);
        form1.print();
        assert(false);
}
void is_same_formula(Formula const& form1, Formula const& form2, Rule const & rule, int idx)
{
    if (form1.length != form2.length)
        __same(form1, form2, rule, idx);

    for (int i = 0; i < form1.length; ++i) 
        if (form1.array[i] != form2.array[i])
            __same(form1, form2, rule, idx);

    for (auto& key : keys)
        if (form1.counts.at(key) != form2.counts.at(key))
            __same(form1, form2, rule, idx);

    for (auto& key : permanent_keys)
        if (form1.permanent_counts.at(key) != form2.permanent_counts.at(key))
            __same(form1, form2, rule, idx);
}

void replace(Formula * p_formula, Rule const & p_rule, int const p_idx, bool reverse)
{
    int const from_size = reverse ? p_rule.to_size   : p_rule.from_size;
    int const to_size   = reverse ? p_rule.from_size : p_rule.to_size;
    int const delta     = to_size - from_size;

    std::string const & new_str = reverse ? p_rule.from : p_rule.to;

    // pad
    if (delta > 0) {
        // FFFTT.....
        // 012345
        // 102<-100,..., 5<-3 
        for (int i = p_formula->length + delta - 1; i >= p_idx + to_size; --i) {
            p_formula->array[i] = p_formula->array[i - delta];
        }
    }
    if (delta < 0) {
        // TTTFF....
        // 012345
        // 3<-5, ... 98<-100 
        for (int i = p_idx + to_size; i < p_formula->length + delta; ++i) {
            p_formula->array[i] = p_formula->array[i - delta];
        }
    }

    p_formula->length += delta;

    // replace
    for (int i = 0; i < to_size; ++i) {
        p_formula->array[p_idx + i] = new_str[i];
    }

    // update keys
    int lambda = reverse ? -1 : 1;
    for (auto const& [key, value] : permanent_counts) {
        p_formula->permanent_counts[key] += lambda * p_rule.permanent_counts.at(key);
    }
    for (auto const& [key, value] : counts) {
        p_formula->counts[key] += lambda * p_rule.counts.at(key);
    }
}

bool can_replace(Formula const * p_formula, Rule const & p_rule, int const p_idx)
{
    bool can = true;
    if (p_formula->length + p_rule.delta > MAX_LENGTH) {
        can = false;
    }
    if (p_formula->length + p_rule.delta < 0) {
        can = false;
    }

    if (can) {
        for (int i = 0; i < p_rule.from_size; ++i) {
            can &= (p_formula->array[p_idx + i] == p_rule.from[i]);
            if (!can) {
                break;
            }
        }
    }

    return can;
}

bool validate(Formula const * p_formula)
{
    bool valid = p_formula->length == MAX_LENGTH;
    if (valid) {
        for (int i = 0; i < MAX_LENGTH; ++i) {
            if (p_formula->array[i] != target[i]) {
                valid = false;
                break;
            }
        }
    }
    return valid;
}

int min_steps = 1'000;
void rec(Formula * p_formula, int const p_steps)
{
    // p_formula->print();
    uint64_t hash = p_formula->get_hash();
    if (seen.find(hash) != seen.end()) {
        return;
    }


    if (p_steps >= min_steps) {
        return;
    }

    for (auto const& [key, value] : permanent_counts) {
        if (p_formula->permanent_counts[key] > value) {
            return;
        }
    }

    for (auto const& [key, value] : counts) {
        if (p_formula->counts[key] > value + 2) {
            return;
        }
    }

    if (validate(p_formula)) {
        min_steps = p_steps;
        std::cout << "valid at " << p_steps << '\n';
        return;
    }

    seen.insert(hash);

    if (seen.size() % 10'000 == 0) {
        std::cerr << "seen " << seen.size() << "\n";
        p_formula->print();
    }

    for (int i = 0; i < p_formula->length; ++i) {
        for (auto const& rule : rules) {
            if (can_replace(p_formula, rule, i)) {
                // Formula copy = *p_formula;
                // replace(p_formula, rule, i);
                // replace(p_formula, rule, i, true);
                // is_same_formula(copy, *p_formula, rule, i);

                replace(p_formula, rule, i);
                rec(p_formula, p_steps + 1);
                replace(p_formula, rule, i, true);
            }
        }
    }
}

void setup_hashes()
{
    srand(time(NULL));
    for (int i = 0; i < MAX_LENGTH; ++i) {
        std::vector<uint64_t> h;
        for (char c = 'A'; c <= 'z'; ++c) {
            h.push_back(rand() << 32 + rand());
        }
        hashes.push_back(h);
    }
}

void setup_count()
{
    // target keys 1
    for (auto& rule : rules) {
        keys.insert(rule.from);
    }

    for (auto& key : permanent_keys) {
        permanent_counts[key] = 0;
        for (int i = 0; i < MAX_LENGTH; ++i) {
            bool valid = true;
            for (int j = 0; j < key.size(); ++j) {
                if (i + j >= MAX_LENGTH || target[i+j] != key[j]) {
                    valid = false;
                }
            }
            if (valid)
                permanent_counts[key]++;
        }
    }

    // target keys 2
    for (auto& key : keys) {
        counts[key] = 0;
        for (int i = 0; i < MAX_LENGTH; ++i) {
            bool valid = true;
            for (int j = 0; j < key.size(); ++j) {
                if (i + j >= MAX_LENGTH || target[i+j] != key[j]) {
                    valid = false;
                }
            }
            if (valid)
                counts[key]++;
        }
    }


    // rules keys 1
    for (auto& rule: rules) {
        for (auto& key : permanent_keys) {
            rule.permanent_counts[key] = 0;
            for (int i = 0; i < rule.to_size; ++i) {
                bool valid = true;
                for (int j = 0; j < key.size(); ++j) {
                    if (i + j >= rule.to_size || rule.to[i+j] != key[j]) {
                        valid = false;
                    }
                }
                if (valid)
                    rule.permanent_counts[key]++;
            }
        }

        for (auto& key : keys) {
            rule.counts[key] = 0;
            for (int i = 0; i < rule.to_size; ++i) {
                bool valid = true;
                for (int j = 0; j < key.size(); ++j) {
                    if (i + j >= rule.to_size || rule.to[i+j] != key[j]) {
                        valid = false;
                    }
                }
                if (valid)
                    rule.counts[key]++;
            }
        }

        rule.counts[rule.from] -= 1;

        std::cerr << rule.from << " => " << rule.to << "\n";
        for (auto& key : keys) {
            if (rule.counts[key]) {
                std::cerr << "  " << key << ": " << rule.counts[key] << "\n";
            }
        }
        std::cerr << "\n";
    }
}

int main()
{
    bool is_react = false;
    std::string line;
    std::string comp;
    while (std::getline(std::cin, line)) {
        if (!is_react) {
            if (line.empty()) {
                is_react = true;
            }
            else {
                Rule rule;
                std::stringstream stream (line);
                stream >> rule.from >> comp >> rule.to;
                rule.setup();
                rules.push_back(rule);
            }
        }

        else {
            target = line;
        }
    }

    MAX_LENGTH = target.size();

    Formula formula;
    formula.setup();

    setup_hashes();
    setup_count();

    rec(&formula, 0);

    std::cerr << "best score " << min_steps << "\n";
}
