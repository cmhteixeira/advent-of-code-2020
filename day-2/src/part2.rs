use super::Entry;
use crate::parse;
use std::ops::BitXor;

pub fn is_valid(input: &str) -> bool {
    let entry: Result<Entry, String> = parse(input);
    let entry = entry.unwrap();

    entry.password.chars().nth((entry.min - 1) as usize).map(|i| i == entry.char).unwrap_or(false).bitxor(
        entry.password.chars().nth((entry.max - 1) as usize).map(|i| i == entry.char).unwrap_or(false)
    )
}

#[cfg(test)]
mod tests {
    use crate::part2::is_valid;

    #[test]
    fn _1_3_a_abcde_is_valid() {
        assert_eq!(is_valid("1-3 a: abcde"), true)
    }

    #[test]
    fn _1_3_b_cdefg_is_valid() {
        assert_eq!(is_valid("1-3 b: cdefg"), false)
    }

    #[test]
    fn _2_9_c_ccccccccc_is_invalid() {
        assert_eq!(is_valid("2-9 c: ccccccccc"), false)
    }

    #[test]
    fn works_on_rightmost() {
        assert_eq!(is_valid("2-10 k: efanvfuask"), true)
    }

    #[test]
    fn works_on_leftmost() {
        assert_eq!(is_valid("1-10 w: wfanvfuask"), true)
    }

    #[test]
    fn false_when_max_greater_than_pass_size() {
        assert_eq!(is_valid("1-10 w: pfanv"), false)
    }

    #[test]
    fn false_when_char_everywhere_except_policy() {
        assert_eq!(is_valid("3-7 w: wwpwwwjwwwww"), false)
    }

    #[test]
    fn true_when_max_greater_than_pass_size_but_char_on_min() {
        assert_eq!(is_valid("3-20 j: sdjaiubfigef"), true)
    }
}
