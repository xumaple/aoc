stack = []
pairs = { '}':'{', ']':'[', ')':'(', '>':'<' }
scores = { ')': 3,']': 57,'}': 1197,'>': 25137 }

def score(line):
    for c in line:
        if c in ['{', '[', '(', '<']:
            stack.append(c)
        else:
            if stack.pop() != pairs[c]:
                return scores[c]
    return 0


with open('input.txt') as f:
    print(sum([score(l.rstrip()) for l in f.readlines()]))