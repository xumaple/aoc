def run(N_DAYS):
    N_DAYS_RESET = 6
    N_DAYS_NEW = N_DAYS_RESET + 2

    c = [0]*(N_DAYS_NEW+1)

    with open('input.txt') as f:
        li = f.readline().rstrip().split(',')
        for i in li:
            c[int(i)] += 1

    from collections import deque

    d = deque(c)

    day = 0
    while day != N_DAYS:
        n = d.popleft()
        d.extend([n])
        d[N_DAYS_RESET] += n

        day += 1

    print(sum(d))

run(80)
run(256)