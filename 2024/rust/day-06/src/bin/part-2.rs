use day_06::process_part2;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("Part 2: {}", process_part2(&input));
}
