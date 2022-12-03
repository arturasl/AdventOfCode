import sys

def main():
    points = 0
    for line in (line.strip() for line in sys.stdin if line):
        m = len(line) // 2
        c = list(set(line[:m]) & set(line[m:]))[0]
        p = ord(c) - (ord('A') - 26 if 'A' <= c <= 'Z' else ord('a')) + 1
        points += p
    print(f'{points}')

if __name__ == "__main__":
    main()
