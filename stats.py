import sys
from statistics import mean, median


def main():
    scores: list[int] = []

    for line in sys.stdin:
        scores.append(int(line))

    print(
        f"Min: {min(scores)}\n"
        + f"Max: {max(scores)}\n"
        + f"Mean: {mean(scores)}\n"
        + f"Median: {median(scores)}\n"
        + f"Total: {sum(scores)}"
    )


if __name__ == "__main__":
    main()
