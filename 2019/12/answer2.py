import re
from sys import stdin
import math

class Point:
    def __init__(self, x = 0, y = 0, z = 0):
        self.x = x
        self.y = y
        self.z = z

    def __str__(self):
        return f"<x={str(self.x).rjust(3)}, y={str(self.y).rjust(3)}, z={str(self.z).rjust(3)}>"

class Planet:
    def __init__(self):
        self.pos = Point()
        self.vel = Point()

    def __str__(self):
        return f"pos={self.pos}, vel={self.vel}"

def parseLine(line):
    reg = re.compile("<x=(-?\d+), y=(-?\d+), z=(-?\d+)>\n")
    res = reg.findall(line)
    return Point(int(res[0][0]), int(res[0][1]), int(res[0][2]))

print("After 0 steps:")
planets = []
for line in stdin:
    pos = parseLine(line)
    planet = Planet()
    planet.pos = pos
    planets.append(planet)
    print(planet)

keys = {}
p = 0
key = ()
for planet in planets:
    key += (planet.pos.x, planet.vel.x)
keys[key] = 1

# for x
while True:
    p += 1
    for i in range(len(planets)):
        for j in range(i+1, len(planets)):
            planet1 = planets[i]
            planet2 = planets[j]

            deltaX = planet2.pos.x - planet1.pos.x
            # deltaX = deltaX // abs(deltaX) if deltaX != 0 else 0
            if deltaX > 0:
                dx = 1
            elif deltaX == 0:
                dx = 0
            else:
                dx = -1
            deltaX = dx
            planet1.vel.x += deltaX
            planet2.vel.x -= deltaX
    key = ()
    for planet in planets:
        planet.pos.x += planet.vel.x
        key += (planet.pos.x, planet.vel.x)
    if key in keys:
        periodX = p
        break
    else:
        keys[key] = 1
print("period for X =", periodX)

keys = {}
p = 0
key = ()
for planet in planets:
    key += (planet.pos.y, planet.vel.y)
keys[key] = 1

# for y
while True:
    p += 1
    for i in range(len(planets)):
        for j in range(i+1, len(planets)):
            planet1 = planets[i]
            planet2 = planets[j]

            deltaY = planet2.pos.y - planet1.pos.y
            # deltaX = deltaX // abs(deltaX) if deltaX != 0 else 0
            if deltaY > 0:
                dy = 1
            elif deltaY == 0:
                dy = 0
            else:
                dy = -1
            deltaY = dy
            planet1.vel.y += deltaY
            planet2.vel.y -= deltaY
    key = ()
    for planet in planets:
        planet.pos.y += planet.vel.y
        key += (planet.pos.y, planet.vel.y)
    if key in keys:
        periodY = p
        break
    else:
        keys[key] = 1
print("period for Y =", periodY)


keys = {}
p = 0
key = ()
for planet in planets:
    key += (planet.pos.z, planet.vel.z)
keys[key] = 1

# for z
while True:
    p += 1
    for i in range(len(planets)):
        for j in range(i+1, len(planets)):
            planet1 = planets[i]
            planet2 = planets[j]

            deltaZ = planet2.pos.z - planet1.pos.z
            # deltaX = deltaX // abs(deltaX) if deltaX != 0 else 0
            if deltaZ > 0:
                dz = 1
            elif deltaZ == 0:
                dz = 0
            else:
                dz = -1
            deltaZ = dz
            planet1.vel.z += deltaZ
            planet2.vel.z -= deltaZ
    key = ()
    for planet in planets:
        planet.pos.z += planet.vel.z
        key += (planet.pos.z, planet.vel.z)
    if key in keys:
        periodZ = p
        break
    else:
        keys[key] = 1
print("period for Z =", periodZ)

lcmXY = periodX * periodY // math.gcd(periodX, periodY)
lcmXYZ = lcmXY * periodZ // math.gcd(lcmXY, periodZ)
print(lcmXYZ)
