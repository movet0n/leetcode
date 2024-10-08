mod easy;

fn main() {
    /* >>> Longest common prefix */
    let words = vec!["flower", "flow", "flight"];
    let result = easy::longest_common_prefix::longest_common_prefix(words);
    println!(">>> Common prefix: {:?}", result);

    /* >>> Palindrom */
    // Option 1
    let num = 121;
    println!(
        "\n>>> Palindrom (option 1): {}",
        easy::is_palindrome::is_palindrome_reversed_self(num)
    );
    // Option 2
    let num = 121;
    println!(
        ">>> Palindrom (option 2): {}",
        easy::is_palindrome::is_palindrome_first_last(num)
    );

    /* >>> Remove duplicated from sorted array */
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let k =
        easy::remove_duplicates_from_sorted_array::remove_duplicates_from_sorted_array(&mut nums);

    println!("\n>>> Number of unique elements: {}", k);
    println!(">>> Array after removing duplicates: {:?}", &nums[..k]);

    /* >>> Two sum */
    let nums = vec![2, 7, 11, 15];
    let target = 22;
    let result = easy::two_sum::two_sum(nums, target);
    println!("\n>>> Two sum: {:?}", result);

    /* >>> Score of string */
    let word = "hello";
    let score = easy::score_of_string::score_of_string(word);
    println!("\n>>> Score of '{}': {}", word, score);

    /* >>> Uncommon words from two sentences */
    let s1 = "this apple is sweet";
    let s2 = "this apple is sour";

    let result = easy::uncommon_words_from_two_sentences::uncommon_words(s1, s2);
    println!("\n>>> Uncommon words: {:?}", result);
}
