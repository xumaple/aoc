h = 0
d = 0
matrix = [h, d]

dirF = ('forward', 0, 1) # horizontal
dirD = ('down', 1, 1) # vertical
dirU = ('up', 1, -1) #vertical
directions = {d[0][0]: d for d in [dirF, dirD, dirU]}

with open('input.txt') as f:
    for l in f.readlines():
        direction_string, matrix_index, multiplier = directions[l[0]]
        num = int(l[len(direction_string)+1:])
        matrix[matrix_index] += num*multiplier

product = 1
for x in matrix:
    product *= x
print(product)