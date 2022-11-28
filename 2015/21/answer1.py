import sys
import itertools

boss_hps = -1
boss_dmg = -1
boss_arm = -1
for line in sys.stdin:
    val = int(line.split()[-1])
    if   boss_hps == -1: boss_hps = val
    elif boss_dmg == -1: boss_dmg = val
    elif boss_arm == -1: boss_arm = val


boss_save = {
    'hps': boss_hps,
    'dmg': boss_dmg,
    'arm': boss_arm
}


def create_item(name, cost, dmg, arm):
    return {
        'name': name,
        'cost': cost,
        'dmg':  dmg,
        'arm':  arm
    }

weapons = [
    create_item('Dagger',        8,     4,       0),
    create_item('Shortsword',   10,     5,       0),
    create_item('Warhammer',    25,     6,       0),
    create_item('Longsword',    40,     7,       0),
    create_item('Greataxe',     74,     8,       0)
]

armors = [
    create_item('Leather',      13,     0,       1),
    create_item('Chainmail',    31,     0,       2),
    create_item('Splintmail',   53,     0,       3),
    create_item('Bandedmail',   75,     0,       4),
    create_item('Platemail',   102,     0,       5)
]

rings = [
    create_item('Damage +1',    25,     1,       0),
    create_item('Damage +2',    50,     2,       0),
    create_item('Damage +3',   100,     3,       0),
    create_item('Defense +1',   20,     0,       1),
    create_item('Defense +2',   40,     0,       2),
    create_item('Defense +3',   80,     0,       3)
]

def simulate(player, boss):
    while True:
        dmg = player['dmg'] - boss['arm']
        if dmg < 1: dmg = 1
        boss['hps'] -= dmg
        if boss['hps'] <= 0:
            return True
        
        dmg = boss['dmg'] - player['arm']
        if dmg < 1: dmg = 1
        player['hps'] -= dmg
        if player['hps'] <= 0:
            return False


min_gold = 10_000

wpn_length = 1
for wpn in itertools.combinations(weapons, wpn_length):
    for arm_length in range(0, 1+1):
        for arm in itertools.combinations(armors, arm_length):
            for rng_length in range(0, 2+1):
                for rng in itertools.combinations(rings, rng_length):
                    player = {}
                    player['hps'] = 100
                    player['dmg'] = 0
                    player['arm'] = 0

                    boss = {}
                    boss['hps'] = boss_save['hps']
                    boss['dmg'] = boss_save['dmg']
                    boss['arm'] = boss_save['arm']

                    gold = 0
                    for w in wpn:
                        player['dmg'] += w['dmg']
                        gold += w['cost']
                    for a in arm:
                        player['arm'] += a['arm']
                        gold += a['cost']
                    for r in rng:
                        player['dmg'] += r['dmg']
                        player['arm'] += r['arm']
                        gold += r['cost']

                    if gold < min_gold and simulate(player, boss):
                        min_gold = gold
                        print("new best", gold)


print(min_gold)