use std::fs;
use advent_of_code_day_1::part1::part1;
use advent_of_code_day_1::part2::part2;
use std::path::Path;

fn main() {
    let contents = fs::read_to_string(Path::new("resource/input.txt"))
        .expect("Something went wrong reading the file");

    let input: Vec<i32> = contents.lines().map(|p|{
        p.parse::<i32>().unwrap()
    }).collect();

    println!("Result part1: {:?}", part1(input.as_slice()));
    println!("Result part2: {:?}", part2(input.as_slice()));
}