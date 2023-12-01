h = 0
d = 0
aim = 0
matrix = [h, d]

dirF = ('forward') # horizontal
dirD = ('down') # vertical
dirU = ('up') #vertical
directions = {d[0][0]: d for d in [dirF, dirD, dirU]}

with open('input.txt') as f:
    for l in f.readlines():
        direction_string = directions[l[0]]
        num = int(l[len(direction_string)+1:])

        if direction_string == dirF:
            matrix[0] += num
            matrix[1] += num*aim
        elif direction_string == dirD:
            aim += num
        else:
            aim -= num

product = 1
for x in matrix:
    product *= x
print(product)