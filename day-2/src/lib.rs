#![feature(try_trait)]

pub mod part1;
pub mod part2;

use regex::Regex;
use std::fmt::Debug;
use std::ops::Try;

#[derive(Debug)]
pub struct Entry<'pass> {
    min: u8,
    max: u8,
    char: char,
    password: &'pass str,
}

fn parse(s: &str) -> Result<Entry, String> {
    let regex_extractor: Regex = Regex::new(r"([0-9]*)-([0-9]*) ([a-z]): ([a-z]*)").unwrap();
    let mut res = regex_extractor.captures_iter(s);
    let res = res
        .next()
        .into_result()
        .map_err(|_e| String::from("No match"))?;

    let min = res
        .get(1)
        .into_result()
        .map_err(|_e| String::from("Minimum not found"))?
        .as_str()
        .parse::<u8>()
        .map_err(|p| String::from("Parsing min to u8"))?;

    let max = res
        .get(2)
        .into_result()
        .map_err(|_e| String::from("Maximum not found"))?
        .as_str()
        .parse::<u8>()
        .map_err(|p| String::from("Parsing max to u8"))?;

    let char = res
        .get(3)
        .into_result()
        .map_err(|_e| String::from("Char not found"))?
        .as_str()
        .parse::<char>()
        .map_err(|p| String::from("Parsing char"))?;

    let password: &str = res
        .get(4)
        .into_result()
        .map_err(|_e| String::from("No password found"))?
        .as_str();

    Ok(Entry {
        min,
        max,
        char,
        password,
    })
}
