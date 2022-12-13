import sys
from collections import defaultdict

def main():
    n = 4
    for line in sys.stdin:
        line = ''.join(['\0'] * (n - 1)) + line.strip()
        u = 0
        v = defaultdict(int)
        for i in range((n - 1), len(line)):
            v[line[i]] += 1
            u += v[line[i]] == 1

            if u == n:
                print(i - (n - 1) + 1)
                break

            v[line[i - (n - 1)]] -= 1
            u -= v[line[i - (n - 1)]] == 0

if __name__ == "__main__":
    main()
