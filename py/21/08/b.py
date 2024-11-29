orders_dict = {
    0: [0,1,2,4,5,6],
    1: [2,5],
    2: [0,2,3,4,6],
    3: [0,2,3,5,6],
    4: [1,2,3,5],
    5: [0,1,3,5,6],
    6: [0,1,3,4,5,6],
    7: [0,2,5],
    8: [0,1,2,3,4,5,6],
    9: [0,1,2,3,5,6],
}

def getVal(order, digits):
    vals_dict = {
        k: set([order[i] for i in v])
        for k, v in orders_dict.items()
    }
    digits = [set(d) for d in digits]
    return int(''.join([next(str(k) for k, v in vals_dict.items() if v == d) for d in digits]))

with open('input.txt') as f:
    total = 0
    for l in f.readlines():
        order = [' ']*7
        clues, digits = l.rstrip().split(' | ')
        clues, digits = clues.split(' '), digits.rstrip().split(' ')

        lengths = {n: [] for n in [2, 3, 4, 5, 6, 7]}
        for s in clues:
            lengths[len(s)].append(set(s))
        
        order[0] = lengths[3][0].difference(l2l := lengths[2][0]).pop()
        order[2] = next(d.pop() for s in lengths[6] if len(d := l2l.difference(n6 := s)) > 0)
        order[5] = l2l.difference({order[2]}).pop()

        dg = lengths[5][0].intersection(lengths[5][1]).intersection(lengths[5][2]).difference({order[0]})
        order[3] = dg.pop()
        order[6] = dg.pop()
        if order[3] not in lengths[4][0]:
            order[6], order[3] = order[3], order[6]
        order[4] = next(c for s in lengths[6] if len(d := n6.difference(s)) > 0 and (c := d.pop()) is not order[3])
        order[1] = n6.difference(set(order)).pop()

        # print(order)

        total += getVal(order, digits)
    print(total)

