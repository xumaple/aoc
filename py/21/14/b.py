from collections import Counter

NUM_STEPS = 40

def step(c, rules):
    new_c = Counter()
    # print(rules)
    for k,v in c.items():
        new_letter = rules.get(k, None)
        # print(k, v, new_letter)
        if new_letter is None:
            new_c[k] += v
        else:
            new_c[(k[0], new_letter)] += v
            new_c[(new_letter, k[1])] += v
    return new_c


with open('input.txt') as f:
    s = f.readline().rstrip()
    f.readline()
    rules = {
        ((pair := l.rstrip().split(' -> '))[0][0], pair[0][1]): pair[1]
        for l in f.readlines()
    }

c = Counter(zip(s[:-1], s[1:]))

for _ in range(NUM_STEPS):
    c = step(c, rules)
    # print(c)

d = Counter(s[-1])
for k,v in c.items():
    d[k[0]] += v
d = sorted(d.values())
# print(d)
print(d[-1] - d[0])