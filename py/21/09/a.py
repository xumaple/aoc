class Map:
    def __init__(self, filename):
        with open(filename) as f:
            self.map = [[int(i) for i in list(l.rstrip())] for l in f.readlines()]
    
    def getVal(self, x, y):
        if x < 0 or x >= len(self.map) or y < 0 or y >= len(self.map[x]):
            return 10
        return self.map[x][y]

    def isLower(self, x, y):
        return (v:=self.getVal(x,y)) < self.getVal(x,y+1) and v < self.getVal(x+1,y) and v < self.getVal(x-1,y) and v < self.getVal(x,y-1)
    
    def getLowerPoints(self):
        return [n for x, l in enumerate(self.map) for y, n in enumerate(l) if self.isLower(x, y)]


m = Map('input.txt')
print(sum([n+1 for n in m.getLowerPoints()]))