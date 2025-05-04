import re
import sys


def main():
    re_line = re.compile(R"^(?P<i1>\d+)-(?P<i2>\d+) (?P<ch>[a-z]): (?P<pswd>[a-z]+)$")

    num_correct = 0
    for line in sys.stdin:
        m = re_line.match(line.strip())
        assert m

        target_ch = m["ch"]
        pswd = m["pswd"]
        num_correct += sum(pswd[int(m[x]) - 1] == target_ch for x in ["i1", "i2"]) == 1

    print(num_correct)


if __name__ == "__main__":
    main()
