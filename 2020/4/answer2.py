import sys
import re

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
            if key + ':' in line:
                if key == 'byr':
                    reg = 'byr:(\w+)'
                    match = re.search(reg, line)
                    try:
                        val = int(match[0][4:])
                        if val >= 1920 and val <= 2002:
                            passport[key] = True
                    except:
                        pass

                if key == 'iyr':
                    reg = key + ':(\w+)'
                    match = re.search(reg, line)
                    try:
                        val = int(match[0][4:])
                        if val >= 2010 and val <= 2020:
                            passport[key] = True
                    except:
                        pass

                if key == 'eyr':
                    reg = key + ':(\w+)'
                    match = re.search(reg, line)
                    try:
                        val = int(match[0][4:])
                        if val >= 2020 and val <= 2030:
                            passport[key] = True
                    except:
                        pass

                if key == 'hgt':
                    reg = key + ':(\w+)cm'
                    match = re.search(reg, line)
                    try:
                        val = int(match[0][4:-2])
                        if val >= 150 and val <= 193:
                            passport[key] = True
                    except:
                        pass
                    reg = key + ':(\w+)in'
                    match = re.search(reg, line)
                    try:
                        val = int(match[0][4:-2])
                        if val >= 59 and val <= 76:
                            passport[key] = True
                    except:
                        pass

                if key == 'hcl':
                    reg = key + ':#(\w+)'
                    match = re.search(reg, line)
                    try:
                        val = match[0][5:]
                        valid = len(val) == 6
                        for v in val:
                            valid = valid and (v >= 'a' and v <= 'z' or v >= '0' and v <= '9')
                        if valid:
                            passport[key] = True
                    except:
                        pass

                if key == 'ecl':
                    reg = key + ':(\w+)'
                    match = re.search(reg, line)
                    try:
                        val = match[0][4:]
                        if val in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']:
                            passport[key] = True
                    except:
                        pass

                if key == 'pid':
                    reg = key + ':(\w+)'
                    match = re.search(reg, line)
                    try:
                        val = match[0][4:]
                        valid = len(val) == 9
                        for v in val:
                            valid = valid and v >= '0' and v <= '9'
                        if valid:
                            passport[key] = True
                    except:
                        pass

print(valid_passports)