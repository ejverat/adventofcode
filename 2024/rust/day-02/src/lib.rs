use std::usize;

// enumeration increase or decrease
enum Direction {
    Increasing,
    Decreasing,
}

pub fn get_level_with_problem(report: &Vec<i32>) -> Option<usize> {
    let mut direction: Option<Direction> = None;
    for i in 1..report.len() {
        // validate the size of increasing/decreasing
        if (report[i] - report[i - 1]).abs() > 3 || report[i] == report[i - 1] {
            return Some(i);
        }
        // validate if is decreasing
        if report[i] < report[i - 1] {
            match direction {
                Some(ref d) => match d {
                    Direction::Increasing => {
                        return Some(i);
                    }
                    _ => {}
                },
                None => direction = Some(Direction::Decreasing),
            }
        } else if report[i] > report[i - 1] {
            match direction {
                Some(ref d) => match d {
                    Direction::Decreasing => {
                        return Some(i);
                    }
                    _ => {}
                },
                None => direction = Some(Direction::Increasing),
            }
        } else {
            return Some(i);
        }
    }
    None
}

pub fn process_part1(input: &str) -> i32 {
    let result: i32 = input
        .lines()
        .map(|report| {
            let v_levels: Vec<i32> = report
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            //check if level are increasing or decreasing
            return get_level_with_problem(&v_levels).is_none();
        })
        .filter(|safety| *safety == true)
        .count() as i32;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_test() {
        let result = process_part1(&INPUT);
        assert_eq!(result, 2);
    }

}
