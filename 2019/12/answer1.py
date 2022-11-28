import re
from sys import stdin

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

for step in range(1000):
    for i in range(len(planets)):
        for j in range(i+1, len(planets)):
            planet1 = planets[i]
            planet2 = planets[j]

            deltaX = planet2.pos.x - planet1.pos.x
            deltaX = deltaX // abs(deltaX) if deltaX != 0 else 0
            planet1.vel.x += deltaX
            planet2.vel.x -= deltaX

            deltaY = planet2.pos.y - planet1.pos.y
            deltaY = deltaY // abs(deltaY) if deltaY != 0 else 0
            planet1.vel.y += deltaY
            planet2.vel.y -= deltaY

            deltaZ = planet2.pos.z - planet1.pos.z
            deltaZ = deltaZ // abs(deltaZ) if deltaZ != 0 else 0
            planet1.vel.z += deltaZ
            planet2.vel.z -= deltaZ

    print(f"After {step+1} steps:")
    for planet in planets:
        planet.pos.x += planet.vel.x
        planet.pos.y += planet.vel.y
        planet.pos.z += planet.vel.z
        print(planet)

energy = 0
for planet in planets:
    pot = abs(planet.pos.x) + abs(planet.pos.y) + abs(planet.pos.z)
    kin = abs(planet.vel.x) + abs(planet.vel.y) + abs(planet.vel.z)
    energy += pot * kin
print("\nEnergy:", energy)