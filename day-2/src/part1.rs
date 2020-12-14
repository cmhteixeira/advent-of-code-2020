use super::Entry;
use crate::parse;

pub fn is_valid(input: &str) -> bool {
    let entry: Result<Entry, String> = parse(input);
    let entry = entry.unwrap();
    let num_chars = entry.password.chars().filter(|char| *char == entry.char).count();

    (num_chars >= entry.min as usize) && (num_chars <= entry.max as usize)
}

