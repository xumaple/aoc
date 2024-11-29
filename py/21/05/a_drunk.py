from collections import OrderedDict
from bisect import bisect_left
h = {}
v = {}
everything = set()

def sort(d):
    return {key:d[key] for key in sorted(d.keys())}

def add_to_set(s, dicts):
    for k, d in dicts:
        for v, n in sort(d):
            

with open('input2.txt') as f:
    for l in f.readlines():
        points = l.rstrip().split(' -> ')
        x1, y1 = points[0].split(',')
        x2, y2 = points[1].split(',')
        
        if x1 == x2:
            a, b = int(y1), int(y2)
            if a > b:
                a, b = b, a
            if len(v) == 0:
                d = OrderedDict()
                d[a] = 1
                d[b] = 0
                v[x1] = d
                continue
            else:
                d = v[x1]
        elif y1 == y2:
            a, b = int(x1), int(x2)
            if a > b:
                a, b = b, a
            if len(h) == 0:
                d = OrderedDict()
                d[a] = 1
                d[b] = 0
                v[y1] = d
                continue
            else:
                d = h[y1]
        else:
            continue
        d = sort(d)
        d, a, b
        keys = list(d.keys())

        l_index = bisect_left(keys, a)
        r_index = bisect_left(keys, b)

        if r_index >= len(keys):
            d[b] = 0
        elif b < keys[r_index]:
            d[b] = d[keys][r_index-1]
        
        print(a, d, l_index)
        if a < keys[l_index]:
            d[a] = d[keys[l_index-1]]
        d = sort(d)
        keys = list(d.keys())
        while keys[l_index] < b:
            print(d, l_index)
            d[keys[l_index]] += 1
            l_index += 1
        
        
        print(d)
        if x1 == x2:
            v[x1] = d
        else:
            h[y1] = d

    # if a[0] 
# add_to_set(everything, h)
add_to_set(everything, v)

print(len(everything))