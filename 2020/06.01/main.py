import sys


def main():
    answer = 0
    for group in "\n".join(line.strip() for line in sys.stdin).split("\n\n"):
        answer += len(set(group) - set(["\n"]))
    print(answer)


if __name__ == "__main__":
    main()
