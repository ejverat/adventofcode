use std::collections::HashMap;

// define a coordenate struct
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    size: Coord,
    guard_location: Coord,
    map: String,
    direction: Direction,
}

fn char_to_direction(c: char) -> Direction {
    match c {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
        _ => panic!("Invalid direction"),
    }
}

fn get_char_at_coord(map: &str, coord: &Coord) -> char {
    let line = map.lines().nth(coord.y as usize).unwrap();
    line.chars().nth(coord.x as usize).unwrap()
}

fn get_next_coord(coord: Coord, direction: &Direction) -> Coord {
    let mut next = coord;
    match direction {
        Direction::Up => next.y -= 1,
        Direction::Down => next.y += 1,
        Direction::Left => next.x -= 1,
        Direction::Right => next.x += 1,
    }

    next.clone()
}

fn is_out_of_bounds(coord: &Coord, map: &Map) -> bool {
    coord.x < 0 || coord.y < 0 || coord.x >= map.size.x || coord.y >= map.size.y
}

fn replace_guard_location(
    multi_line_string: &mut String,
    line_number: usize,
    col_number: usize,
    replacement_char: char,
) {
    let mut lines: Vec<&str> = multi_line_string.lines().collect();

    if let Some(line) = lines.get_mut(line_number) {
        let mut chars: Vec<char> = line.chars().collect();

        if col_number < chars.len() {
            chars[col_number] = replacement_char;
            let new_line: String = chars.iter().collect();
            lines[line_number] = new_line.as_str();
            *multi_line_string = lines.join("\n");
        } else {
            println!("Column position out of range.");
        }
    } else {
        println!("Line number out of range.");
    }
}

fn rotate_direction_to_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

pub fn process_part1(input: &str) -> u64 {
    let mut map = Map {
        size: Coord { x: 0, y: 0 },
        guard_location: Coord { x: 0, y: 0 },
        map: String::from(input),
        direction: Direction::Up,
    };

    map.size.y = map.map.lines().count() as i64;
    map.size.x = map.map.lines().nth(0).unwrap().chars().count() as i64;

    println!("Map size: {:?}", map.size);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' || c == '>' || c == 'v' || c == '<' {
                map.guard_location = Coord {
                    x: x as i64,
                    y: y as i64,
                };
                map.direction = char_to_direction(c);
                break;
            }
        }
    }

    println!("Guard location: {:?}", map.guard_location);
    println!("Direction: {:?}", map.direction);

    while !is_out_of_bounds(&map.guard_location, &map) {
        replace_guard_location(
            &mut map.map,
            map.guard_location.y as usize,
            map.guard_location.x as usize,
            'x',
        );
        let next_location = get_next_coord(map.guard_location, &map.direction);

        if !is_out_of_bounds(&next_location, &map) {
            println!("Guard location: {:?}", map.guard_location);
            println!(
                "Next location: {:?} with char: {}",
                next_location,
                get_char_at_coord(&map.map, &next_location)
            );
            println!("Direction: {:?}", map.direction);
            if get_char_at_coord(&map.map, &next_location) == '#' {
                map.direction = rotate_direction_to_right(map.direction);
            } else {
                map.guard_location = next_location;
            }
        } else {
            map.guard_location = next_location;
        }
    }

    println!("Map: {:?}", map);

    //count 'x' in the string
    map.map.chars().filter(|x| *x == 'x').count() as u64
}

pub fn process_part2(input: &str) -> u64 {
    let mut map = Map {
        size: Coord { x: 0, y: 0 },
        guard_location: Coord { x: 0, y: 0 },
        map: String::from(input),
        direction: Direction::Up,
    };

    map.size.y = map.map.lines().count() as i64;
    map.size.x = map.map.lines().nth(0).unwrap().chars().count() as i64;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' || c == '>' || c == 'v' || c == '<' {
                map.guard_location = Coord {
                    x: x as i64,
                    y: y as i64,
                };
                map.direction = char_to_direction(c);
                break;
            }
        }
    }

    let mut alternatives: Vec<Coord> = Vec::new();
    let start_location = map.guard_location.clone();
    let start_direction = map.direction.clone();

    //go through all map with '.', '<', '^', '>' or 'v'
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                let possible_alter = Coord {
                    x: x as i64,
                    y: y as i64,
                };
                println!("Possible alter: {:?}", possible_alter);
                let mut is_loop = false;
                let mut know_positions_directions: HashMap<Coord, Vec<Direction>> = HashMap::new();

                map.direction = start_direction.clone();
                map.guard_location = start_location.clone();
                while !is_out_of_bounds(&map.guard_location, &map) {
                    // check if current guard location is in know_positions_directions
                    if know_positions_directions.contains_key(&map.guard_location) {
                        // if yes, check if the direction is the same as the known direction
                        if know_positions_directions.get(&map.guard_location).unwrap().contains(&map.direction)
                        {
                            is_loop = true;
                            println!(
                                "Loop found using {:?} of size {:?}",
                                possible_alter, map.size
                            );
                            // print know positions
                            // println!("Possible positions: {:?}", know_positions_directions);
                            break;
                        } else {
                            know_positions_directions.get_mut(&map.guard_location).unwrap().push(map.direction.clone());
                            // println!("Not the same direction at alternative {:?}", possible_alter);
                        }
                    }
                    else {
                        know_positions_directions.insert(map.guard_location, vec![map.direction.clone()]);
                    }
                    let next_location = get_next_coord(map.guard_location, &map.direction);

                    if !is_out_of_bounds(&next_location, &map) {
                        // println!("Guard location: {:?}", map.guard_location);
                        // println!(
                        //     "Next location: {:?} with char: {}",
                        //     next_location,
                        //     get_char_at_coord(&map.map, &next_location)
                        // );
                        // println!("Direction: {:?}", map.direction);
                        if get_char_at_coord(&map.map, &next_location) == '#'
                            || next_location == possible_alter
                        {
                            map.direction = rotate_direction_to_right(map.direction);
                        } else {
                            map.guard_location = next_location;
                        }
                    } else {
                        map.guard_location = next_location;
                    }
                }

                if is_loop {
                    alternatives.push(possible_alter);
                } else {
                    println!(
                        "No loop found using {:?} of size {:?}",
                        possible_alter, map.size
                    );
                }
            }
        }
    }

    //count alternatives
    alternatives.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 6);
    }
}
