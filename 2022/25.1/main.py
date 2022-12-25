import sys
import re

def s2d(x):
    m = 1
    r = 0
    for v in x[::-1]:
        r += {'0': 0, '1': 1, '2': 2, '-': -1, '=': -2}[v] * m
        m *= 5
    return r

def d2f(x):
    m = 1
    while m <= x:
        m *= 5

    r = ''
    while m:
        r += str(x // m)
        x = x % m
        m //= 5

    return r

def d2s(x):
    f = [int(x) for x in list(d2f(x))[::-1]]
    i = 0
    while i < len(f):
        if f[i] >= 5:
            if i + i >= len(f):
                f.append(0)
            f[i + 1] += f[i] // 5
            f[i] = f[i] % 5

        if f[i] == 3:
            f[i] = -2
            f[i + 1] +=1
        elif f[i] == 4:
            f[i] = -1
            f[i + 1] += 1

        i += 1

    while len(f) > 1 and f[-1] == 0:
        f.pop()
    f = f[::-1]
    return ''.join({0: '0', 1: '1', 2: '2', -1: '-', -2: '='}[x] for x in f)

def main():
    # for i in list(range(10 + 1)) + [15, 20, 2022, 12345, 314159265]:
    #     print(f'{i} -> {d2f(i)} -> {d2s(i)}')
    s = sum(s2d(line.strip()) for line in sys.stdin)
    print(d2s(s))

if __name__ == "__main__":
    main()
