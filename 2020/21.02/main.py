# 1 allergen -> 1 ingredient.
# 1 ingredient -> 0..1 allergens.
# if allergen not listed ingredient might still have it.

import copy
import re
import sys
from collections import defaultdict
from collections.abc import Iterable
from dataclasses import dataclass
from typing import TypeVar


@dataclass
class Row:
    ingredients: set[str]
    alergens: set[str]


def read() -> list[Row]:
    re_line = re.compile(r"^((?:\w+)(?: \w+)*)(?: \(contains (.*)\))?$")
    result: list[Row] = []
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        m = re_line.match(line)
        assert m

        ingredients = m[1].split(" ")
        s_ingredients = set(ingredients)
        assert len(ingredients) == len(s_ingredients)

        alergens = m[2].split(", ") if m[2] else []
        s_alergens = set(alergens)
        assert len(s_alergens) == len(alergens)

        result.append(Row(s_ingredients, s_alergens))
    return result


K = TypeVar("K")
E = TypeVar("E")


def rem(d: dict[K, set[E]], k: K, e: E):
    d[k].remove(e)
    if not d[k]:
        del d[k]


def first(i: Iterable[E]) -> E:
    return next(iter(i))


def main():
    rows = read()

    all_ingredients: set[str] = {
        ingredient for row in rows for ingredient in row.ingredients
    }
    all_alergens: set[str] = {alergen for row in rows for alergen in row.alergens}

    alergen_to_ingredients: dict[str, set[str]] = {}
    for alergen in all_alergens:
        alergen_to_ingredients[alergen] = copy.deepcopy(all_ingredients)

    for row in rows:
        for alergen in row.alergens:
            alergen_to_ingredients[alergen] = (
                alergen_to_ingredients[alergen] & row.ingredients
            )

    ingredient_to_alergens: dict[str, set[str]] = defaultdict(set)
    for alergen, ingredients in alergen_to_ingredients.items():
        for ingredient in ingredients:
            ingredient_to_alergens[ingredient].add(alergen)

    num_alergens_to_ingredient: dict[int, set[str]] = defaultdict(set)
    for ingredient, alergens in ingredient_to_alergens.items():
        num_alergens_to_ingredient[len(alergens)].add(ingredient)

    while num_alergens_to_ingredient:
        assert 1 in num_alergens_to_ingredient
        ingredient = first(num_alergens_to_ingredient[1])
        assert len(ingredient_to_alergens[ingredient]) == 1
        alergen = first(ingredient_to_alergens[ingredient])
        rem(num_alergens_to_ingredient, 1, ingredient)

        for other_ingredient in list(alergen_to_ingredients[alergen]):
            if other_ingredient == ingredient:
                continue

            num_alergens = len(ingredient_to_alergens[other_ingredient])
            rem(num_alergens_to_ingredient, num_alergens, other_ingredient)

            ingredient_to_alergens[other_ingredient].remove(alergen)
            alergen_to_ingredients[alergen].remove(other_ingredient)
            num_alergens_to_ingredient[num_alergens - 1].add(other_ingredient)

    print(
        ",".join(
            [
                first(ingredients)
                for _, ingredients in sorted(alergen_to_ingredients.items())
            ]
        )
    )


if __name__ == "__main__":
    main()
