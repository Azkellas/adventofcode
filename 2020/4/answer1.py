import sys


valid_passports = 0
passport = {}
keys = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid', 'cid']

for line in sys.stdin:
    line = line[:-1]
    if line == '':
        # new passport
        valid = True
        for key in keys:
            if not key in passport and key != 'cid':
                valid = False
                break
        if valid:
            valid_passports += 1
        passport = {}
    else:
        for key in keys:
            if key in line:
                passport[key] = True

print(valid_passports)