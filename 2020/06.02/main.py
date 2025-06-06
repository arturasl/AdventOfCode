import sys


def main():
    answer = 0
    for group in "\n".join(line.strip() for line in sys.stdin).split("\n\n"):
        yeses = {chr(c) for c in range(ord("a"), ord("z") + 1)}
        for person in group.split("\n"):
            yeses = yeses & set(person)
        answer += len(yeses)
    print(answer)


if __name__ == "__main__":
    main()
