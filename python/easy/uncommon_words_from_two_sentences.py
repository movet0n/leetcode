from collections import Counter
from typing import List


def uncommon_words(s1: str, s2: str) -> List[str]:
    words = s1.split() + s2.split()
    return [word for word in words if words.count(word) == 1]


def uncommon_words_counter(s1: str, s2: str) -> List[str]:
    words = s1.split() + s2.split()
    word_count = Counter(words)
    return [word for word, count in word_count.items() if count == 1]
