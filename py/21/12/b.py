# NOTE: THIS IS A GARBAGE SOLUTION.
# The DFS generates duplicate paths. That's why I just used a set to discredit the duplicates. Very time inefficient.
# And memory inefficient. And brainspace inefficient. And bad. :(

from collections import defaultdict

FILENAME = 'input.txt'

class Map():
    def __init__(self, _big_caves = None, _small_caves = None, _curr_loc = None, use_freebie = None):
        if _big_caves is None:
            self.big_caves = defaultdict(dict)
            self.small_caves = defaultdict(dict)

            with open(FILENAME) as f:
                for l in f.readlines():
                    a, b = l.rstrip().split('-')
                    self.add_connection(a, b)
                    self.add_connection(b, a)
            return

        self.big_caves = {k: {
            loc: count+1 if loc == _curr_loc and not use_freebie else count
            for loc, count in s.items()
        } for k, s in _big_caves.items()}
        self.small_caves = {k: {
            loc: count+1 if loc == _curr_loc and not use_freebie else count
            for loc, count in s.items()
        } for k, s in _small_caves.items()}

    def _get_map(self, loc):
        return self.small_caves if loc.islower() else self.big_caves

    def add_connection(self, a, b):
        if a == 'end' or b == 'start':
            return
        self._get_map(a)[a][b] = 0

    def deepcopy(self, curr_loc, use_freebie):
        return Map(self.big_caves, self.small_caves, curr_loc, use_freebie)
    
    def get_next_locations(self, curr_loc):
        # print(curr_loc)
        return [loc for loc, count in self._get_map(curr_loc)[curr_loc].items() if (loc.islower() and count < 1) or loc.isupper()]
        
paths = set()
def traverse_paths(map, curr, path, freebie_used):
    global num_paths
    if curr == 'end':
        # print(path[:-3])
        paths.add(path)
        return
    for l in map.get_next_locations(curr):
        traverse_paths(map.deepcopy(curr, False), l, path + l, freebie_used)
        if not freebie_used:
            traverse_paths(map.deepcopy(curr, True), l, path + l, True)


traverse_paths(Map(), 'start', '', False)
print(len(paths))