def is_palindrome_reversed_self(x: int) -> bool:
    return str(x) == str(x)[::-1]


def is_palindrome_first_last(x: int) -> bool:
    if x <= 0:
        return False

    number = str(x)
    half_length = len(number) // 2
    equal_nums = 0
    for _ in range(half_length):
        if number[0] == number[-1]:
            number = number[1:-1]
            equal_nums += 1

    if equal_nums == half_length:
        return True
