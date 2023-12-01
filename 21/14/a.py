from collections import Counter

NUM_STEPS = 10

def step(s, rules):
    return ''.join([a if b == ' ' else a+rules.get(a+b, '') for a, b in zip(s, s[1:]+' ')])

with open('input.txt') as f:
    s = f.readline().rstrip()
    f.readline()
    rules = {
        (pair := l.rstrip().split(' -> '))[0]: pair[1]
        for l in f.readlines()
    }

for _ in range(NUM_STEPS):
    # print(len(s))
    s = step(s, rules)

c = sorted(Counter(s).values())
print(c[-1] - c[0])
