BINGO_LINE_SIZE = 5
BINGO_BOARD_SIZE = BINGO_LINE_SIZE ** 2

class Board:
    def __init__(self):
        self.values = {}
        self.lines = [0]*12
        # indices:
        # 0-4: rows
        # 5-9: columns
        # 10: diag1
        # 11: diag2

    def init(self, f):
        row = 0
        while (l := f.readline().rstrip()) != '':
            # print(l)
            thisRow = [int(x) for x in l.split(' ') if x != '']
            self.values.update([(val, (row, col)) for col, val in enumerate(thisRow)])
            row += 1
        # print(self.values)
        if len(self.values) != BINGO_BOARD_SIZE:
            if len(self.values) != 0:
                print('Error: found', len(self.values))
            return False
        # TODO
        return True

    def _add_row(self, index):
        self.lines[index] += 1
        return self.lines[index] == BINGO_LINE_SIZE

    def calculate_points(self, input):
        self.points = sum(self.values.keys())*input

    def play(self, input):
        try:
            row, col = self.values[input]
        except KeyError:
            return False
        del self.values[input]
        done = False
        if self._add_row(row) or self._add_row(col + BINGO_LINE_SIZE):
            done = True
        if row == col and self._add_row(2*BINGO_LINE_SIZE):
            done = True
        if row + col == BINGO_LINE_SIZE - 1 and self._add_row(2*BINGO_LINE_SIZE + 1):
            done = True
        if done:
            self.calculate_points(input)
        return done
    
    def getPoints(self):
        return self.points

with open('input.txt') as f:
    inputs = [int(x) for x in f.readline().rstrip().split(',')]
    f.readline()

    boards = []
    while True:
        b = Board()
        if not b.init(f):
            break
        boards.append(b)

    # print(boards[0].values)
    for i in inputs:
        for b in boards:
            if b.play(i):
                print(b.getPoints())
                exit()
    