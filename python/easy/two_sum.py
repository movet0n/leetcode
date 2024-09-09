def two_sum(nums, target):
    num_map = {}

    for i, num in enumerate(nums):
        remainder = target - num

        if remainder in num_map:
            return [num_map[remainder], i]

        num_map[num] = i
