from __future__ import annotations

import copy
import math
import re
import sys
from dataclasses import dataclass
from typing import override


@dataclass()
class Tile:
    name: int
    plane: list[list[bool]]

    def h_flip(self: Tile) -> Tile:
        result = Tile(name=self.name, plane=[])
        result.plane = [row[::-1] for row in copy.deepcopy(self.plane)]
        return result

    def v_flip(self: Tile) -> Tile:
        result = Tile(name=self.name, plane=[])
        result.plane = copy.deepcopy(self.plane)[::-1]
        return result

    def clockwise(self: Tile) -> Tile:
        result = Tile(name=self.name, plane=[])
        for col in range(len(self.plane[0])):
            new_row: list[bool] = []
            for row in self.plane[::-1]:
                new_row.append(row[col])
            result.plane.append(new_row)
        return result

    def all_orientations(self: Tile) -> list[Tile]:
        tiles: list[Tile] = []
        tiles.append(copy.deepcopy(self))
        for _ in range(3):
            tiles.append(tiles[-1].clockwise())
        for i in range(4):
            tiles.append(tiles[i].h_flip())
        for i in range(4):
            tiles.append(tiles[i].v_flip())
        return tiles

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
        tiles.append(current_tile)

    return tiles


def fit(
    grid: list[list[Tile | None]], y: int, x: int, possible_tiles: list[list[Tile]]
) -> int:
    r = 1
    if y == len(grid):
        assert grid[0][0] and grid[0][-1] and grid[-1][0] and grid[-1][-1]
        print(grid[0][0].name * grid[0][-1].name * grid[-1][0].name * grid[-1][-1].name)
        return r

    for i, tile_set in enumerate(possible_tiles):
        if not tile_set:
            continue
        possible_tiles[i] = []

        for tile in tile_set:
            # Fits?
            fits = True
            if x != 0:
                lhs = grid[y][x - 1]
                assert lhs
                for rowidx, row in enumerate(lhs.plane):
                    fits = fits and row[-1] == tile.plane[rowidx][0]

            if y != 0:
                above = grid[y - 1][x]
                assert above
                fits = fits and above.plane[-1] == tile.plane[0]

            if not fits:
                continue

            grid[y][x] = tile

            ny, nx = y, x + 1
            if nx == len(grid[ny]):
                ny, nx = ny + 1, 0
            r += fit(grid, ny, nx, possible_tiles)

            grid[y][x] = None

        possible_tiles[i] = tile_set
    return r


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
    print(fit(grid, 0, 0, possible_tiles))


if __name__ == "__main__":
    main()
