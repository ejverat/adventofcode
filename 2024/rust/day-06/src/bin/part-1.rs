use day_06::process_part1;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("Part 1: {}", process_part1(&input));
}
