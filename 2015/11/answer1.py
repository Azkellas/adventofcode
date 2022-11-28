code = input()


def increment(code):
    code = list(code)
    for i in range(1, 8):
        if code[-i] != 'z':
            next_letter = chr(ord(code[-i]) + 1)
            code[-i] = next_letter
            return ''.join(code)
        else:
            code[-i] = 'a'

def valid_inc_straight(code):
    for i in range(8 - 2):
        if ord(code[i]) == ord(code[i+1]) - 1 == ord(code[i+2]) - 2:
            return True
    return False

def valid_ilo(code):
    return not 'i' in code and not 'l' in code and not 'o' in code

def valid_pairs(code):
    i = 0
    count = 0
    while i < 7:
        if code[i] == code[i+1]:
            count += 1
            i += 1
            if count == 2:
                return True
        i += 1
    return False

def validate_code(code):
    return valid_ilo(code) and valid_inc_straight(code) and valid_pairs(code)

while True:
    if validate_code(code):
        print(code)
        exit()
    code = increment(code)