nums = [2, 3, 4, 7]

total = 0
with open('input.txt') as f:
    for l in f.readlines():
        s = l.rstrip().split('|')[1]
        for segment in s.split(' '):
            if len(segment) in nums:
                total += 1

print(total)