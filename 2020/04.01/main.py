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
    num_valid = 0
    for entry in "\n".join(line.strip() for line in sys.stdin).split("\n\n"):
        fields = set([])
        for kv in entry.split():
            assert kv
            parts = kv.split(":")
            assert len(parts) == 2
            fields.add(parts[0])
        num_valid += fields & REQUIRED_FIELDS == REQUIRED_FIELDS
    print(num_valid)


if __name__ == "__main__":
    main()
