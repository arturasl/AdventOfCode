import sys

def main():
    points = 0
    for theirs, mine in ((ord(a) - ord(b) for a, b in zip(line.strip().split(), 'AX')) for line in sys.stdin if line):
        mine = (theirs + mine - 1) % 3
        points += (mine + 1) + ((mine - theirs + 1) % 3) * 3
    print(f'{points}')

if __name__ == "__main__":
    main()
