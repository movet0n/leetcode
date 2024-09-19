def score_of_string(word: str) -> int:

    total = 0
    for i in range(len(word) - 1):
        adjacent_sum = abs(ord(word[i]) - ord(word[i + 1]))
        total += adjacent_sum

    return total


def score_of_string_in_place(word: str) -> int:
    return sum(abs(ord(word[i]) - ord(word[i + 1])) for i in range(len(word) - 1))
