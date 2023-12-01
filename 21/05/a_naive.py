from collections import Counter

def avo(x):
    if x == 0:
        return 0
    return 1 if x > 0 else -1

c = Counter()
with open('input.txt') as f:
    for l in f.readlines():
        points = l.rstrip().split(' -> ')
        x1, y1 = points[0].split(',')
        x2, y2 = points[1].split(',')
        x1, y1, x2, y2 = int(x1), int(y1), int(x2), int(y2)

        if x1 != x2 and y1 != y2:
            continue

        x_increment = avo(x2 - x1)
        y_increment = avo(y2 - y1)

        while True:
            c[(x1, y1)] += 1

            if x1 == x2 and y1 == y2:
                break
            y1 += y_increment
            x1 += x_increment

    print(len([n for n in c.values() if n > 1]))
