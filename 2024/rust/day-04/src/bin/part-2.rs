use day_04::process_part2;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", process_part2(&input));
}
