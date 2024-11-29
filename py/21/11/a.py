class Map:
    def __init__(self, filename):
        with open(filename) as f:
            self.map = []
            self.num_flashes = 0
            self.curr_flashing = set()
            for l in f.readlines():
                self.map.append([int(i) for i in l.rstrip()])

    def oob(self, i, j): # Out of bounds
        return i < 0 or i >= len(self.map) or j < 0 or j >= len(self.map[i])

    def simulate_single(self, i, j):
        if self.oob(i, j):
            return
        self.map[i][j] += 1
        if self.map[i][j] == 10:
            self.num_flashes += 1
            self.curr_flashing.add((i,j))
            self.simulate_single(i, j+1)
            self.simulate_single(i, j-1)
            self.simulate_single(i+1, j)
            self.simulate_single(i-1, j)
            self.simulate_single(i+1, j+1)
            self.simulate_single(i+1, j-1)
            self.simulate_single(i-1, j+1)
            self.simulate_single(i-1, j-1)
        
        
    def simulate(self):
        s = set()
        for i, l in enumerate(self.map):
            for j, _ in enumerate(l):
                self.simulate_single(i, j)
        for i, j in self.curr_flashing:
            self.map[i][j] = 0
        self.curr_flashing.clear()


m = Map('input.txt')
for i in range(100):
    m.simulate()
print(m.num_flashes)