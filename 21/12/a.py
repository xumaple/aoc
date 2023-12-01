from collections import defaultdict

FILENAME = 'input.txt'

class Map():
    def __init__(self, _big_caves = None, _small_caves = None, _curr_loc = None):
        if _big_caves is None:
            self.big_caves = defaultdict(set)
            self.small_caves = defaultdict(set)

            with open(FILENAME) as f:
                for l in f.readlines():
                    a, b = l.rstrip().split('-')
                    self.add_connection(a, b)
                    self.add_connection(b, a)
            return

        if _curr_loc.islower():
            self.big_caves = {k: s-{_curr_loc} for k, s in _big_caves.items()}
            self.small_caves = {k: s-{_curr_loc} for k, s in _small_caves.items()}
        else:
            self.big_caves = {k:v for k,v in _big_caves.items()}    
            self.small_caves = {k:v for k,v in _small_caves.items()}    

    def _get_map(self, loc):
        return self.small_caves if loc.islower() else self.big_caves

    def add_connection(self, a, b):
        if a == 'end' or b == 'start':
            return
        self._get_map(a)[a].add(b)

    def deepcopy(self, curr_loc):
        return Map(self.big_caves, self.small_caves, curr_loc)
    
    def get_next_locations(self, curr_loc):
        return list(self._get_map(curr_loc)[curr_loc])
        
num_paths = 0
def traverse_paths(map, curr):
    global num_paths
    if curr == 'end':
        num_paths += 1
        return
    for l in map.get_next_locations(curr):
        traverse_paths(map.deepcopy(curr), l)

traverse_paths(Map(), 'start')
print(num_paths)