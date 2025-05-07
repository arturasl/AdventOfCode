import sys


def main():
    seats = []
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

        seats.append(row * 8 + col)

    seats = sorted(seats)
    identified = []
    for i in range(1, len(seats) - 1):
        if seats[i] - 1 != seats[i - 1]:
            identified.append(seats[i] - 1)

    assert len(identified) == 1
    print(identified[0])


if __name__ == "__main__":
    main()
