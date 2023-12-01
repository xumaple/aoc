window_size = 3
with open('input.txt') as f:
    arr = [int(l) for l in f.readlines()]
    currS = sum(arr[0:window_size])
    n = 0
    for index, next in enumerate(arr[window_size:]):
        increase = next - arr[index]
        if increase > 0:
            n += 1
        currS += increase
print(n)
