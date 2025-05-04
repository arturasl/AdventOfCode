import re
import sys


def main():
    re_line = re.compile(R"^(?P<mi>\d+)-(?P<ma>\d+) (?P<ch>[a-z]): (?P<pswd>[a-z]+)$")

    num_correct = 0
    for line in sys.stdin:
        m = re_line.match(line.strip())
        assert m

        target_ch = m["ch"]
        num_cnt = sum(ch == target_ch for ch in m["pswd"])
        num_correct += int(m["mi"]) <= num_cnt <= int(m["ma"])

    print(num_correct)


if __name__ == "__main__":
    main()
