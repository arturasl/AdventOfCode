import sys

DIRS = [(dy, dx) for dy in [-1, 0, 1] for dx in [-1, 0, 1] if (dy, dx) != (0, 0)]


def find_visible(
    around: list[tuple[int, int]], mp_cur: list[list[str]]
) -> list[list[list[str]]]:
    visible: list[list[list[str]]] = [[[] for _ in row] for row in mp_cur]

    for y, x in around:
        for dy, dx in DIRS:
            cy, cx = y, x
            cur_ch = None

            while True:
                cy += dy
                cx += dx
                if not (1 <= cy < len(mp_cur) - 1 and 1 <= cx < len(mp_cur[cy]) - 1):
                    break
                if cur_ch:
                    visible[cy][cx].append(cur_ch)
                if mp_cur[cy][cx] != ".":
                    cur_ch = mp_cur[cy][cx]

    return visible


def main():
    mp_cur = [["."] + list(line.strip()) + ["."] for line in sys.stdin if line]
    empty_row = ["." for _ in range(len(mp_cur[0]))]
    mp_cur.append(empty_row)
    mp_cur.insert(0, empty_row)
    mp_next = [["." for _ in row] for row in mp_cur]

    around: list[tuple[int, int]] = []
    around += [(0, x) for x in range(len(mp_cur[0]))]
    around += [(len(mp_cur) - 1, x) for x in range(len(mp_cur[0]))]
    around += [(y, 0) for y in range(1, len(mp_cur) - 1)]
    around += [(y, len(mp_cur[y]) - 1) for y in range(1, len(mp_cur) - 1)]

    changed = True
    while changed:
        changed = False
        visible = find_visible(around, mp_cur)

        for y in range(1, len(mp_cur) - 1):
            for x in range(1, len(mp_cur[y]) - 1):
                mp_next[y][x] = mp_cur[y][x]
                if mp_cur[y][x] == "L":
                    if all(seat != "#" for seat in visible[y][x]):
                        changed = True
                        mp_next[y][x] = "#"
                elif mp_cur[y][x] == "#":
                    if sum(seat == "#" for seat in visible[y][x]) >= 5:
                        changed = True
                        mp_next[y][x] = "L"

        mp_cur, mp_next = mp_next, mp_cur

    print(sum(sum(seat == "#" for seat in row) for row in mp_cur))


if __name__ == "__main__":
    main()
