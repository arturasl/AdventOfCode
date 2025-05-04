import re
import sys

REQUIRED_FIELDS = frozenset(
    [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ]
)


def main():
    re_num = re.compile(r"^\d+$")
    res = {
        "byr": re_num,
        "iyr": re_num,
        "eyr": re_num,
        "hgt": re.compile(r"^(?P<num>\d+)(?P<m>cm|in)$"),
        "hcl": re.compile(r"^#[a-z0-9]{6}$"),
        "ecl": re.compile(r"^(amb|blu|brn|gry|grn|hzl|oth)$"),
        "pid": re_num,
    }
    assert res.keys() == REQUIRED_FIELDS

    checks = {
        "byr": lambda _, v: 1920 <= int(v) <= 2002,
        "iyr": lambda _, v: 2010 <= int(v) <= 2020,
        "eyr": lambda _, v: 2020 <= int(v) <= 2030,
        "hgt": lambda m, _: (
            False
            or (m["m"] == "cm" and 150 <= int(m["num"]) <= 193)
            or (m["m"] == "in" and 59 <= int(m["num"]) <= 76)
        ),
        "hcl": lambda *_: True,
        "ecl": lambda *_: True,
        "pid": lambda _, v: len(v) == 9,
    }
    assert checks.keys() == REQUIRED_FIELDS

    num_valid = 0
    for entry in "\n".join(line.strip() for line in sys.stdin).split("\n\n"):
        missing_fields = set(REQUIRED_FIELDS)
        for kv in entry.split():
            assert kv
            field, val = kv.split(":")

            if field not in missing_fields:
                assert field not in REQUIRED_FIELDS
                continue

            m = res[field].match(val)
            if not m:
                continue

            if checks[field](m, val):
                missing_fields.remove(field)

        num_valid += not missing_fields
    print(num_valid)


if __name__ == "__main__":
    main()
