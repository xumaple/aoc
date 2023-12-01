from functools import total_ordering
from heapq import heappush, heappop

@total_ordering
class Position:
    def __init__(self, val, x, y):
        self.val = val
        self.x = x
        self.y = y
    
    def __eq__(self, other):
        return self.val == other.val

    def __lt__(self, other):
        return self.val < other.val

    def isFinal(self, x, y):
        return self.x == x and self.y == y

with open('input.txt') as f:
    map = [[int(x) for x in l.rstrip()]for l in f.readlines()]

final_x = len(map)-1
final_y = len(map[final_x])-1

curr_best = [[9999999999999999 for _ in l] for l in map]
curr_best[0][0] = 0

heap = [Position(0, 0, 0)]

def add_to_heap(curr_val, x, y, heap):
    if x < 0 or x >= len(map) or y < 0 or y >= len(map[x]):
        return
    new_val = curr_val + map[x][y]
    if new_val < curr_best[x][y]:
        curr_best[x][y] = new_val
        heappush(heap, Position(new_val, x, y))

while True:
    if heap[0].isFinal(final_x, final_y):
        print(heap[0].val)
        break
    p = heappop(heap)
    add_to_heap(p.val,p.x,p.y+1,heap)
    add_to_heap(p.val,p.x,p.y-1,heap)
    add_to_heap(p.val,p.x+1,p.y,heap)
    add_to_heap(p.val,p.x-1,p.y,heap)


