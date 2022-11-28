class Amp:
    def __init__(self, codes):
        self.codes = [int(i) for i in codes.split(',')]
        self.index = 0
        self.relativeBase = 0
        self.inputs = []

        self.validOpcodes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 99]
        self.operations = [
            None,           # 0
            self.add,       # 1
            self.mult,      # 2
            self.input,     # 3
            self.output,    # 4
            self.jumpIf,    # 5
            self.jumpIfNot, # 6
            self.lessThan,  # 7
            self.equal,     # 8
            self.switchBase # 9
        ]

    def reset(self, codes):
        self.index = 0
        self.relativeBase = 0
        for i in range(len(codes)):
            if self.codes[i] != codes[i]:
                self.codes[i] = codes[i]
        for i in range(len(codes), len(self.codes)):
            self.codes[i] = 0

    def getWriteIndex(self, idx, mode):
        if mode == 0:
            self.safeMemory(idx)
            return idx
        elif mode == 2:
            self.safeMemory(self.relativeBase + idx)
            return self.relativeBase + idx
        else:
            print(f"Wrong mode in getWriteIndex: {mode}")

    def getReadVal(self, idx, mode):
        if mode == 0:
            self.safeMemory(idx)
            return self.codes[idx]
        elif mode == 1:
            return idx
        elif mode == 2:
            self.safeMemory(self.relativeBase + idx)
            return self.codes[self.relativeBase + idx]
        else:
            print(f"Wrong mode in getReadVal: {mode}")


    def getVals(self, getTypes):
        opcode = str(self.codes[self.index])
        vals = []
        for i in range(len(getTypes)):
            idx = self.codes[self.index + i + 1]

            mode = int(opcode[-3-i] if len(opcode) >= 3+i else 0)

            getType = getTypes[i]
            getter = [self.getReadVal, self.getWriteIndex][getType == 'write']

            vals.append(getter(idx, mode))
        return vals

    def add(self):
        # opcode = 1
        [val1, val2, tar] = self.getVals(['read', 'read', 'write'])
        self.codes[tar] = val1 + val2
        self.index += 4

    def mult(self):
        # opcode = 2
        [val1, val2, tar] = self.getVals(['read', 'read', 'write'])
        self.codes[tar] = val1 * val2
        self.index += 4

    def input(self):
        # opcode = 3
        if len(self.inputs) == 0:
            return True  # halts

        [tar] = self.getVals(['write'])
        self.codes[tar] = self.inputs.pop(0)
        self.index += 2

    def output(self):
        # opcode = 4
        [val] = self.getVals(['read'])
        self.index += 2
        return val

    def jumpIf(self):
        # opcode = 5
        [val1, val2] = self.getVals(['read', 'read'])
        self.index = [self.index+3, val2][val1 != 0]

    def jumpIfNot(self):
        # opcode = 6
        [val1, val2] = self.getVals(['read', 'read'])
        self.index = [self.index+3, val2][val1 == 0]

    def lessThan(self):
        # opcode = 7
        [val1, val2, tar] = self.getVals(['read', 'read', 'write'])
        self.codes[tar] = val1 < val2
        self.index += 4

    def equal(self):
        # opcode = 8
        [val1, val2, tar] = self.getVals(['read', 'read', 'write'])
        self.codes[tar] = val1 == val2
        self.index += 4

    def switchBase(self):
        # opcode = 9
        [val] = self.getVals(['read'])
        self.relativeBase += val
        self.index += 2


    def safeMemory(self, idx):
        if idx >= len(self.codes):
            needed = idx + 1 - len(self.codes)
            for i in range(needed):
                self.codes.append(0)

    def run(self):
        while True:
            opcode = str(self.codes[self.index])
            op = int(opcode[-2:])

            if not op in self.validOpcodes:
                print(f'Error: weird op = {op} ')

            if op == 99:
                # end program
                print("end")
                return None
            elif op == 4:
                  return self.operations[op]()
            else:
                halt = self.operations[op]()
                if halt:
                    print("halt")
                    return None

    def runByStep(self, steps=1):
        for step in range(steps):
            opcode = str(self.codes[self.index])
            op = int(opcode[-2:])

            if not op in self.validOpcodes:
                print(f'Error: weird op = {op} ')

            if op == 99:
                # end program
                print("end")
                return "END"
            elif op == 4:
                  return self.operations[op]()
            else:
                halt = self.operations[op]()
                if halt:
                    print("halt")
                    return "HALT"
