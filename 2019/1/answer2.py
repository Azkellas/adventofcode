import math
from sys import stdin

totalFuel = 0
currentFuel = 0

def fuelNeeded(mass):
    fuel = 0
    while mass > 0:
        print(mass, end=" ")
        newMass = math.floor(mass / 3) - 2
        fuel += newMass if newMass > 0 else 0
        mass = newMass
        # print(mass)
    return fuel

print(fuelNeeded(100756))

for line in stdin:
    mass = int(line)
    totalFuel += fuelNeeded(mass)
    print(totalFuel)
print(totalFuel)

