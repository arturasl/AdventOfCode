import sys

ADJECENT = [(dy, dx) for dy in [-1, 0, 1] for dx in [-1, 0, 1] if (dy, dx) != (0, 0)]


def main():
    mp_cur = [["."] + list(line.strip()) + ["."] for line in sys.stdin if line]
    empty_row = ["." for _ in range(len(mp_cur[0]))]
    mp_cur.append(empty_row)
    mp_cur.insert(0, empty_row)
    mp_next = [["." for _ in row] for row in mp_cur]

    changed = True
    while changed:
        changed = False
        for y in range(1, len(mp_cur) - 1):
            for x in range(1, len(mp_cur[y]) - 1):
                adjecent: list[str] = []
                for dy, dx in ADJECENT:
                    adjecent.append(mp_cur[y + dy][x + dx])

                mp_next[y][x] = mp_cur[y][x]
                if mp_cur[y][x] == "L":
                    if all(seat != "#" for seat in adjecent):
                        changed = True
                        mp_next[y][x] = "#"
                elif mp_cur[y][x] == "#":
                    if sum(seat == "#" for seat in adjecent) >= 4:
                        changed = True
                        mp_next[y][x] = "L"

        mp_cur, mp_next = mp_next, mp_cur

    print(sum(sum(seat == "#" for seat in row) for row in mp_cur))


if __name__ == "__main__":
    main()
