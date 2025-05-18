import sys


def main():
    lines = [line for line in [line.strip() for line in sys.stdin] if line]
    min_start = int(lines[0])
    busses = [int(bus) for bus in lines[1].split(",") if bus != "x"]
    waits = [
        {
            "wait": (min_start // bus + bool(min_start % bus)) * bus - min_start,
            "id": bus,
        }
        for bus in busses
    ]
    best = min(waits, key=lambda w: w["wait"])

    print(best["wait"] * best["id"])


if __name__ == "__main__":
    main()
