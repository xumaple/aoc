class Trie:
    def __init__(self, val):
        self.d = {}
        self.sizes = {'0': 0, '1': 0}
        self.val = val
    
    def add(self, s):
        if len(s) == 0:
            return
        trie = self.d.get(s[0], Trie(s[0]))
        trie.add(s[1:])
        self.d[s[0]] = trie
        self.sizes[s[0]] += 1
    
    def getGreater(self):
        if len(self.d) == 1:
            return list(self.d.values())[0]
        # print(len(self.d), self.sizes)
        return self.d['0'] if self.sizes['0'] > self.sizes['1'] else self.d['1']

    def getSmaller(self):
        if len(self.d) == 1:
            return list(self.d.values())[0]
        # print(len(self.d), self.sizes)
        return self.d['0'] if self.sizes['0'] <= self.sizes['1'] else self.d['1']

    def empty(self):
        return self.sizes['0'] + self.sizes['1'] == 0

t = Trie(None)
with open('input.txt') as f:
    for s in f.readlines():
        t.add(s.rstrip())

oxyT = t.getGreater()
oxyArr = ''
while not oxyT.empty():
    oxyArr += oxyT.val
    oxyT = oxyT.getGreater()
oxyArr += oxyT.val
print(oxyArr)

coT = t.getSmaller()
coArr = ''
while not coT.empty():
    coArr += coT.val
    coT = coT.getSmaller()
coArr += coT.val
print(coArr)

print(int(oxyArr, 2)* int(coArr, 2))