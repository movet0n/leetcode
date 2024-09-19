pub fn score_of_string(word: &str) -> i32 {
    let mut total = 0;

    let chars: Vec<u8> = word.bytes().collect();
    println!("{:?}", chars);

    for i in 0..chars.len() - 1 {
        let adjacent_sum = (chars[i] as i32 - chars[i + 1] as i32).abs();
        total += adjacent_sum;
    }

    return total;
}
