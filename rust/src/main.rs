mod easy;

fn main() {
    //
    // Palindrom
    //
    // Option 1
    let num = 121;
    println!(
        ">>> Palindrom (option 1): {}",
        easy::is_palindrome::is_palindrome_reversed_self(num)
    );
    // Option 2
    let num = 121;
    println!(
        ">>> Palindrom (option 2): {}",
        easy::is_palindrome::is_palindrome_first_last(num)
    );

    //
    // Two sum
    //
    let nums = vec![2, 7, 11, 15];
    let target = 22;
    let result = easy::two_sum::two_sum(nums, target);
    println!(">>> Two sum: {:?}", result);
}
