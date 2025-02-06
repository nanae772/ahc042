import sys
from statistics import mean, median


def main():
    scores: list[int] = []

    for line in sys.stdin:
        scores.append(int(line))

    print(
        f"Min: {min(scores)}\nMax: {max(scores)}\nMean: {mean(scores)}\nMedian: {median(scores)}"
    )


if __name__ == "__main__":
    main()
