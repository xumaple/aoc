from queue import SimpleQueue as Queue

class Map:
    def __init__(self, filename):
        with open(filename) as f:
            self.map = [[int(i) for i in list(l.rstrip())] for l in f.readlines()]
            self.tracked = [[False]*len(l) for l in self.map]
            self.basins = []
    
    def outOfBounds(self, x, y):
        return x < 0 or x >= len(self.map) or y < 0 or y >= len(self.map[x])

    def getVal(self, x, y):
        return self.map[x][y] if not self.outOfBounds(x,y) else 10

    def isLower(self, x, y):
        return (v:=self.getVal(x,y)) < self.getVal(x,y+1) and v < self.getVal(x+1,y) and v < self.getVal(x-1,y) and v < self.getVal(x,y-1)
    
    def getLowerPoints(self):
        return [(x,y) for x, l in enumerate(self.map) for y, n in enumerate(l) if self.isLower(x, y)]

    def addToQueue(self, q, x,y):
        if self.getVal(x,y) >= 9 or self.tracked[x][y]:
            return
        self.tracked[x][y] = True
        q.put((x,y))
        return
    
    def fillBasin(self, x,y):
        size = 0
        q = Queue()
        self.addToQueue(q, x, y)


        while not q.empty():
            x,y = q.get()
            size += 1
            self.addToQueue(q, x, y+1)
            self.addToQueue(q, x+1, y)
            self.addToQueue(q, x, y-1)
            self.addToQueue(q, x-1, y)
        
        self.basins.append(size)
        return

    def getBasinSizes(self):
        for x, y in self.getLowerPoints():
            if not self.tracked[x][y]:
                self.fillBasin(x,y)
        
        return self.basins
        


m = Map('input.txt')
print((s := sorted(m.getBasinSizes(), reverse=True))[0] * s[1] * s[2])