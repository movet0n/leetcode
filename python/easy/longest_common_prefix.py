from typing import List


def longest_common_prefix(words: List[str]) -> str:
    if not words:
        return ""

    prefix = words[0]

    s: str
    for s in words[1:]:
        while not s.startswith(prefix):
            prefix = prefix[:-1]
            if not prefix:
                return ""

    return prefix


def longest_common_prefix_zip(words):
    if not words:
        return ""

    for i, letter_group in enumerate(zip(*words)):
        if len(set(letter_group)) > 1:
            return words[0][:i]

    return min(words, key=len)
