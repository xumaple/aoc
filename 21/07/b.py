
input = [int(i) for i in open('input.txt').readline().rstrip().split(',')]

def sum_to_n(n):
    return int(n*(n+1)/2)


m = int(sum(input)/len(input))
print(sum([sum_to_n(abs(m-n)) for n in input]))