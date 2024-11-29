stack = []
pairs = { '}':'{', ']':'[', ')':'(', '>':'<' }
scores = { '(': 1,'[': 2,'{': 3,'<': 4 }

def score(lines):
    for l in lines:
        if s := score_helper(l.rstrip()):
            yield s
        else:
            stack.clear()
def score_helper(line):
    total = 0
    for c in line:
        if c in ['{', '[', '(', '<']:
            stack.append(c)
        else:
            if stack.pop() != pairs[c]:
                return total
    while len(stack):
        total *= 5
        total += scores[stack.pop()]
    return total

def median(l):
    index = (len(l)-1)//2
    if len(l)%2 ==0:
        return (l[index]+l[index+1])//2
    return l[index]


with open('input.txt') as f:
    print(median(sorted(score(f.readlines()))))