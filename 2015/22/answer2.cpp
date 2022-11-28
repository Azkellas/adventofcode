#include <iostream>
#include <unordered_map>

enum Winner {
    HERO,
    BOSS,
    NOT_OVER
};

enum Costs {
     MISSILE_COST =  53,
       DRAIN_COST =  73,
      SHIELD_COST = 113,
      POISON_COST = 173,
    RECHARGE_COST = 229
};

enum Cooldowns {
      SHIELD_CD = 6,
      POISON_CD = 6,
    RECHARGE_CD = 5
};

enum Damages {
     MISSILE_DMG =   4,
       DRAIN_DMG =   2,
      SHIELD_DMG =   7,
      POISON_DMG =   3,
    RECHARGE_DMG = 101
};

// input data
int HERO_HP   =  50;
int HERO_MANA = 500;
int BOSS_HP   =  55;
int BOSS_DMG  =   8;

enum Spell {
    FIRST_SPELL,

    MISSILE     = FIRST_SPELL,
    DRAIN,
    SHIELD,
    POISON,
    RECHARGE,

    LAST_SPELL
};

std::ostream& operator<<(std::ostream& os, Spell const spell)
{
    switch(spell) {
        case MISSILE:  os << "Missile";  break;
        case DRAIN:    os << "Drain";    break;
        case SHIELD:   os << "Shield";   break;
        case POISON:   os << "Poison";   break;
        case RECHARGE: os << "Recharge"; break;
    }
    return os;
}

class State
{
public:
    int hero_hp {HERO_HP};
    int hero_mana {HERO_MANA};

    int boss_hp {BOSS_HP}; // from input
    int boss_dmg {BOSS_DMG}; // from input

    int shield_cd {0};
    int poison_cd {0};
    int recharge_cd {0};

    int mana_spent {0};

    void apply_effects() {
        if (shield_cd > 0)
            --shield_cd;

        if (poison_cd > 0) {
            --poison_cd;
            boss_hp -= 3;
        }

        if (recharge_cd > 0) {
            --recharge_cd;
            hero_mana += 101;
            if (hero_mana > 500) hero_mana = 500;
        }
    }

    inline bool can_missile()  const { return hero_mana >=  MISSILE_COST;                               }
    inline bool can_drain()    const { return hero_mana >=    DRAIN_COST;                               }
    inline bool can_shield()   const { return hero_mana >=   SHIELD_COST &&   shield_cd == 0; }
    inline bool can_poison()   const { return hero_mana >=   POISON_COST &&   poison_cd == 0; }
    inline bool can_recharge() const { return hero_mana >= RECHARGE_COST && recharge_cd == 0; }


    void missile() {
        hero_mana  -= MISSILE_COST;
        boss_hp    -= MISSILE_DMG;
        mana_spent += MISSILE_COST;
    }

    void drain() {
        hero_mana  -= DRAIN_COST;
        boss_hp    -= DRAIN_DMG;
        hero_hp    += DRAIN_DMG;
        if (hero_hp > HERO_HP) hero_hp = HERO_HP;
        mana_spent += DRAIN_COST;
    }

    void shield() {
        hero_mana  -= SHIELD_COST;
        shield_cd   = SHIELD_CD;
        mana_spent += SHIELD_COST;
    }

    void poison() {
        hero_mana  -= POISON_COST;
        poison_cd   = POISON_CD;
        mana_spent += POISON_COST;
    }

    void recharge() {
        hero_mana  -= RECHARGE_COST;
        recharge_cd   = RECHARGE_CD;
        mana_spent += RECHARGE_COST;
    }

    void boss_attack() {
        hero_hp -= std::max(1, boss_dmg - (shield_cd ? SHIELD_DMG : 0));
    }

    inline Winner get_winner() const {
        return boss_hp <= 0
            ? HERO
            : hero_hp <= 0
                ? BOSS
                : NOT_OVER;
    }

    inline bool can_spell(Spell const spell) {
        switch (spell) {
            case MISSILE:  return can_missile();
            case DRAIN:    return can_drain();
            case SHIELD:   return can_shield();
            case POISON:   return can_poison();
            case RECHARGE: return can_recharge();
            default:
                return false;
        }
    }

    inline void do_spell(Spell const spell) {
        switch (spell) {
            case MISSILE:  return missile();
            case DRAIN:    return drain();
            case SHIELD:   return shield();
            case POISON:   return poison();
            case RECHARGE: return recharge();
            default:
                return;
        }
    }
};

int MIN_MANA = 1e7;

int play_turn(State p_state, int const depth)
{
    if (p_state.mana_spent > MIN_MANA) {
        return p_state.mana_spent;
    }

    int min_mana = 1e7;

    // P2
    p_state.hero_hp--;
    if (p_state.get_winner() == BOSS) {
        return p_state.mana_spent;
    }

    p_state.apply_effects();

    for (int i_spell = 0; i_spell < LAST_SPELL; ++i_spell) {
        Spell spell = (Spell)i_spell;
        if (p_state.can_spell(spell)) {
            // for (int i = 0; i < depth; ++i) std::cerr << "| ";
            // std::cerr << spell << ": " << p_state.hero_hp << " - " << p_state.boss_hp << "\n";
            State copy = p_state;
            copy.do_spell(spell);
            if (copy.get_winner() == HERO) {
                // std::cerr << "============================     winning           ======================\n";
                if (copy.mana_spent < MIN_MANA) {
                    MIN_MANA = copy.mana_spent;
                    std::cerr << "new best: " << copy.mana_spent << '\n';
                    min_mana = std::min(min_mana, copy.mana_spent);
                }
            }
            else {
                copy.apply_effects();
                if (copy.get_winner() == HERO) {
                // std::cerr << "============================     winning           ======================\n";
                    if (copy.mana_spent < MIN_MANA) {
                        MIN_MANA = copy.mana_spent;
                        std::cerr << "new best: " << copy.mana_spent << '\n';
                        min_mana = std::min(min_mana, copy.mana_spent);
                    }
                }
                else {
                    copy.boss_attack();
                    if (copy.get_winner() == NOT_OVER)
                        min_mana = std::min(min_mana, play_turn(copy, depth + 1));
                }
            }
        }
    }

    return min_mana;
}

int main()
{
    // test data;
    // HERO_HP   = 10;
    // HERO_MANA = 250;
    // BOSS_HP   = 14;
    // BOSS_DMG  = 8;

    State state;
    play_turn(state, 0);

    std::cout << "Min mana: " << MIN_MANA << "\n";
}
