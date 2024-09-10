use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let remainder = target - num;

        if let Some(&index) = num_map.get(&remainder) {
            return vec![index as i32, i as i32];
        }

        num_map.insert(num, i);
    }

    return vec![];
}
