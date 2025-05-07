import sys


def main():
    mx = 0
    for line in sys.stdin:
        line = line.strip()
        assert len(line) == 10

        row = 0
        for c in line[:7]:
            assert c in "FB"
            row = (row << 1) | (c == "B")

        col = 0
        for c in line[7:10]:
            assert c in "LR"
            col = (col << 1) | (c == "R")

        mx = max(mx, row * 8 + col)
    print(mx)


if __name__ == "__main__":
    main()
