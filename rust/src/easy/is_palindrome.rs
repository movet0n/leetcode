pub fn is_palindrome_reversed_self(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let s = x.to_string();
    return s.chars().eq(s.chars().rev());
}

pub fn is_palindrome_first_last(x: i32) -> bool {
    if x <= 0 {
        return false;
    }

    let mut number = x.to_string();
    let half_length = number.len() / 2;
    let mut equal_nums = 0;

    for _ in 0..half_length {
        if number.chars().nth(0) == number.chars().nth(number.len() - 1) {
            number = number.chars().skip(1).take(number.len() - 2).collect();
            equal_nums += 1;
        }
    }

    return equal_nums == half_length;
}
