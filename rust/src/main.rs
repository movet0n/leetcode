mod easy;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 22;
    let result = easy::two_sum::two_sum(nums, target);
    println!("{:?}", result);
}
