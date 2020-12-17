use std::fs;
use std::path::Path;
use day_3::number_trees;


fn main() {
    let contents = fs::read_to_string(Path::new("resources/input.txt"))
        .expect("Something went wrong reading the file");

    // println!("{}", contents.as_str());
    println!("Number of trees is: {}", number_trees(contents.as_str()));
}