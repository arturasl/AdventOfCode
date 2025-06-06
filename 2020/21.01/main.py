# 1 allergen -> 1 ingredient.
# 1 ingredient -> 0..1 allergens.
# if allergen not listed ingredient might still have it.

import copy
import re
import sys
from dataclasses import dataclass


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

    ingrediends_with_alergens = {
        ingredient
        for ingredients in alergen_to_ingredients.values()
        for ingredient in ingredients
    }
    ingredients_wo_alergens = all_ingredients - ingrediends_with_alergens

    print(sum(len(row.ingredients & ingredients_wo_alergens) for row in rows))


if __name__ == "__main__":
    main()
