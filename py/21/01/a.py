with open('input.txt') as f:
    curr = -999999999999
    n = -1
    for l in f.readlines():
        i = int(l)
        if i > curr:
            n += 1
        curr = i
print(n)
