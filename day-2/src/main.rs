use std::fs;
use std::path::Path;
use advent_of_code_day_2::part1::{is_valid as part1};
use advent_of_code_day_2::part2::{is_valid as part2};

fn main() {
    let contents = fs::read_to_string(Path::new("resource/input.txt"))
        .expect("Something went wrong reading the file");


    let res_part1 = contents.lines().fold(0, |acc, i| {
        if part1(i) {acc + 1}
        else {acc}
    });

    let res_part2 = contents.lines().fold(0, |acc, i| {
        if part2(i) {acc + 1}
        else {acc}
    });

    println!("{:?}", res_part1);
    println!("{:?}", res_part2);
}