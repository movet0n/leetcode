from typing import List


def remove_duplicates_from_sorted_array(nums: List[int]) -> int:
    if not nums:
        return 0

    k = 1

    for i in range(1, len(nums)):
        if nums[i] != nums[i - 1]:
            nums[k] = nums[i]
            k += 1

    return k
