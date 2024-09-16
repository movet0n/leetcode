pub fn remove_duplicates_from_sorted_array(nums: &mut Vec<i32>) -> usize {
    if nums.is_empty() {
        return 0;
    }

    let mut k = 1;

    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            nums[k] = nums[i];
            k += 1;
        }
    }

    return k;
}
