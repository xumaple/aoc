import numpy as np

with open('input.txt') as f:
    v = f.readlines()
    print(len(v[0]))
    arr = np.zeros(len(v[0])-1)
    total = len(v)
    for row in v:
        arr = np.add(arr, np.array([int(c) for c in row.rstrip()]))
    print(total, arr)

gamma = 0
epsilon = 0

multiplier = 1
for x in arr[::-1]:
    if x > total/2:
        gamma += multiplier
    else:
        epsilon += multiplier
    multiplier *= 2

print(gamma*epsilon)