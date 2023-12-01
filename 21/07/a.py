
input = sorted([int(i) for i in open('input.txt').readline().rstrip().split(',')])

def median(l):
    index = (len(l)-1)//2
    if len(l)%2 ==0:
        return (l[index]+l[index+1])/2
    return l[index]

m = int(median(input))
print(sum([abs(m-n) for n in input]))