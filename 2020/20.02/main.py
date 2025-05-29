from __future__ import annotations

import copy
import math
import re
import sys
from dataclasses import dataclass
from typing import TypeVar, override

TILE_DIM = 10

T = TypeVar("T")


def h_flip(grid: list[list[T]]) -> list[list[T]]:
    return [row[::-1] for row in copy.deepcopy(grid)]


def v_flip(grid: list[list[T]]) -> list[list[T]]:
    return copy.deepcopy(grid)[::-1]


def clockwise(grid: list[list[T]]) -> list[list[T]]:
    result: list[list[T]] = []
    for col in range(len(grid[0])):
        new_row: list[T] = []
        for row in grid[::-1]:
            new_row.append(row[col])
        result.append(new_row)
    return result


def all_orientations(grid: list[list[T]]) -> list[list[list[T]]]:
    result: list[list[list[T]]] = []
    result.append(copy.deepcopy(grid))
    for _ in range(3):
        result.append(clockwise(result[-1]))
    for i in range(4):
        result.append(h_flip(result[i]))
    for i in range(4):
        result.append(v_flip(result[i]))
    return result


@dataclass()
class Tile:
    name: int
    plane: list[list[bool]]

    def h_flip(self: Tile) -> Tile:
        result = Tile(name=self.name, plane=[])
        result.plane = h_flip(self.plane)
        return result

    def v_flip(self: Tile) -> Tile:
        result = Tile(name=self.name, plane=[])
        result.plane = v_flip(self.plane)
        return result

    def clockwise(self: Tile) -> Tile:
        result = Tile(name=self.name, plane=[])
        result.plane = clockwise(self.plane)
        return result

    def all_orientations(self: Tile) -> list[Tile]:
        tiles: list[Tile] = [
            Tile(self.name, plane) for plane in all_orientations(self.plane)
        ]

        return list({str(tile): tile for tile in tiles}.values())

    @override
    def __str__(self: Tile) -> str:
        result = f"Tile {self.name}:\n"
        result += "\n".join(
            ["".join(["#" if x else "." for x in row]) for row in self.plane]
        )
        return result


def read_tiles() -> list[Tile]:
    tiles: list[Tile] = []
    current_tile: Tile | None = None

    re_tile_id = re.compile(r"\d+")

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue

        if "Tile" in line:
            if current_tile:
                assert current_tile.plane
                tiles.append(current_tile)
            tile_id = re_tile_id.search(line)
            assert tile_id
            current_tile = Tile(name=int(tile_id[0]), plane=[])
            continue

        assert current_tile
        current_tile.plane.append([x == "#" for x in line])

    if current_tile:
        assert current_tile.plane
        assert len(current_tile.plane) == TILE_DIM
        tiles.append(current_tile)

    return tiles


def does_fit(grid: list[list[Tile | None]], y: int, x: int, tile: Tile) -> bool:
    if x != 0:
        lhs = grid[y][x - 1]
        assert lhs
        for rowidx, row in enumerate(lhs.plane):
            if row[-1] != tile.plane[rowidx][0]:
                return False

    if y != 0:
        above = grid[y - 1][x]
        assert above
        if above.plane[-1] != tile.plane[0]:
            return False

    return True


def find_dragons(grid: list[list[Tile | None]]):
    str_grid = [
        ["x" for _ in range(len(grid) * (TILE_DIM - 2))]
        for _ in range(len(grid) * (TILE_DIM - 2))
    ]

    for gy, row in enumerate(grid):
        for gx, tile in enumerate(row):
            assert tile
            for ty in range(1, TILE_DIM - 1):
                for tx in range(1, TILE_DIM - 1):
                    str_grid[gy * (TILE_DIM - 2) + ty - 1][
                        gx * (TILE_DIM - 2) + tx - 1
                    ] = "#" if tile.plane[ty][tx] else "."

    pat = ["                  # ", "#    ##    ##    ###", " #  #  #  #  #  #   "]

    for orien in all_orientations(str_grid):
        num_found = 0
        for y, orow in enumerate(orien):
            for x, _ in enumerate(orien[y]):
                ok = True
                for py, prow in enumerate(pat):
                    for px, pchar in enumerate(prow):
                        ok = ok and y + py < len(orien)
                        ok = ok and x + px < len(orow)
                        ok = ok and (pchar != "#" or orien[y + py][x + px] == "#")

                if ok:
                    for py, prow in enumerate(pat):
                        for px, pchar in enumerate(prow):
                            if pchar == "#":
                                orien[y + py][x + px] = "O"
                num_found += ok
        if num_found:
            print("\n".join("".join(row) for row in orien))
            print(sum(sum(c == "#" for c in row) for row in orien))


def fit(
    grid: list[list[Tile | None]], y: int, x: int, possible_tiles: list[list[Tile]]
) -> bool:
    if y == len(grid):
        find_dragons(grid)
        return True

    for i, tile_set in enumerate(possible_tiles):
        if not tile_set:
            continue
        possible_tiles[i] = []

        for tile in tile_set:
            if not does_fit(grid, y, x, tile):
                continue

            grid[y][x] = tile

            ny, nx = y, x + 1
            if nx == len(grid[ny]):
                ny, nx = ny + 1, 0
            if fit(grid, ny, nx, possible_tiles):
                return True

            grid[y][x] = None

        possible_tiles[i] = tile_set

    return False


def main():
    tiles = read_tiles()
    possible_tiles: list[list[Tile]] = []
    for tile in tiles:
        possible_tiles.append([])
        for orientation in tile.all_orientations():
            possible_tiles[-1].append(orientation)

    width = math.isqrt(len(tiles))
    assert width**2 == len(tiles)
    grid: list[list[Tile | None]] = [[None for _ in range(width)] for _ in range(width)]
    _ = fit(grid, 0, 0, possible_tiles)


if __name__ == "__main__":
    main()
