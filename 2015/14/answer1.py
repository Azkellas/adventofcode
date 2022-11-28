import sys


max_time = 2503

max_dist = 0
for line in sys.stdin:
    line = line.split()
    speed = int(line[3])
    moving = int(line[6])
    resting = int(line[-2])

    state = 'moving'
    time = 0
    dist = 0
    while time < max_time:
        if state == 'moving':
            moving = min(moving, max_time - time)
            time += moving
            dist += speed * moving
            state = 'resting'
        else:
            time += resting
            state = 'moving'
    if dist > max_dist:
        max_dist = dist

print(max_dist)