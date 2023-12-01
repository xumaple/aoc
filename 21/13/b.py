points = []
folds = []

def reflect_point(p, fold):
    axis, flip_point = fold
    if (d := (p[0] if axis == 'x' else p[1])) > flip_point:
        return (2*flip_point-d, p[1]) if axis == 'x' else (p[0], 2*flip_point-d)
    return p

def combine_points(points, fold):
    return list(set([reflect_point(p, fold) for p in points]))

with open('input.txt') as f:
    for l in f.readlines():
        if l == '\n':
            continue
        if l[0] != 'f':
            x,y = l.rstrip().split(',')
            points.append((int(x),int(y)))
        else:
            x = l.rstrip().split('along ')[1]
            folds.append((x[0], int(x[2:])))

for f in folds:
    points = combine_points(points, f)

max_x = max([x for x,y in points])+1
max_y = max([y for x,y in points])+1

grid = [[' ']*max_x for _ in range(max_y)]
for x,y in points:
    # print(x,y, grid[y])
    grid[y][x] = '#'

for l in grid:
    print(''.join(l))
    