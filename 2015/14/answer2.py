import sys


max_time = 2503

reindeers = []
for line in sys.stdin:
    line = line.split()
    speed = int(line[3])
    moving = int(line[6])
    resting = int(line[-2])

    state = 'moving'
    time = 0
    dist = 0

    reindeers.append({
        'state': 'moving',
        'speed': speed,
        'moving': moving,
        'resting': resting,
        'cooldown': moving,
        'score': 0,
        'dist': 0
    })


for time in range(max_time):
    max_dist = -1
    for reindeer in reindeers:
        reindeer['cooldown'] -= 1
        if reindeer['state'] == 'moving':
            reindeer['dist'] += reindeer['speed']

        if reindeer['cooldown'] == 0:
            reindeer['state'] = ['moving', 'resting'][reindeer['state'] == 'moving']
            reindeer['cooldown'] = [reindeer['resting'], reindeer['moving']][reindeer['state'] == 'moving']
        
        if reindeer['dist'] > max_dist:
            max_dist = reindeer['dist']
    
    for reindeer in reindeers:
        if reindeer['dist'] == max_dist:
            reindeer['score'] += 1
    

max_score = 0
for reindeer in reindeers:
    if reindeer['score'] > max_score:
        max_score = reindeer['score']

print(max_score)
