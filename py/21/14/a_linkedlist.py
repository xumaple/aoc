from collections import Counter

NUM_STEPS = 10

class LinkedString:
    class Node:
        def __init__(self, val = None, next = None):
            self.val = val
            self.next = next

        def step_next(self, rules):
            next = self.next
            if next is not None and (v := rules.get(self.val+next.val, None)) is not None:
                self.next = LinkedString.Node(v, next)
            return next

    class Iterator:
        def __init__(self, curr_node):
            self.curr_node = curr_node

        def __next__(self):
            if self.curr_node is None:
                raise StopIteration
            val = self.curr_node.val
            self.curr_node = self.curr_node.next
            return val
        
    def __init__(self, s = ''):
        if s != '':
            next = None
            for c in s[::-1]:
                next = LinkedString.Node(c, next)
            self.begin = next
        else:
            self.begin = None

    def __iter__(self):
        return LinkedString.Iterator(self.begin)

    def __str__(self):
        # return ''.join(self)
        return ''.join([s for s in self])

    def __len__(self):
        return len([s for s in self])


def step(s, rules):
    iter = s.begin
    while iter is not None:
        iter = iter.step_next(rules)

with open('input.txt') as f:
    s = LinkedString(f.readline().rstrip())
    f.readline()
    rules = {
        (pair := l.rstrip().split(' -> '))[0]: pair[1]
        for l in f.readlines()
    }

for i in range(NUM_STEPS):
    step(s, rules)
    # print(i, len(s))

c = sorted(Counter(s).values())
print(c[-1] - c[0])

