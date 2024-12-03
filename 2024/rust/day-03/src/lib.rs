pub fn process_part1(input: &str) -> u32 {
    const START: &str = "mul(";
    let mut mult_indices = input.match_indices(START);

    let mut result: u32 = 0;

    while let Some((i, _)) = mult_indices.next() {
        let start_mult = &input[(i + START.len())..];
        if let Some(end_i) = start_mult.find(")") {
            let inside_args = &start_mult[..end_i];

            let numbers = inside_args
                .split(",")
                .map(|s| s.parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>();

            if numbers.len() == 2 {
                result += numbers[0] * numbers[1];
            }
        }
    }
    result
}

pub fn process_part2(input: &str) -> u32 {
    const START_MULT: &str = "mul(";
    const START_DO: &str = "do()";
    const START_DONT: &str = "don't()";
    let do_indices = input.match_indices(START_DO);
    let dont_indices = input.match_indices(START_DONT);
    let mut mult_indices = input.match_indices(START_MULT).filter(|x| {
        let closest_minor_dont = dont_indices.clone().filter(|d| d < x).max();
        let closest_minor_do = do_indices.clone().filter(|d| d < x).max();

        if closest_minor_dont.is_none() {
            return true;
        }

        if closest_minor_do.is_none() {
            return false;
        }

        if closest_minor_dont.unwrap() > closest_minor_do.unwrap() {
            return false;
        }

        true
    });

    let mut result: u32 = 0;

    while let Some((i, _)) = mult_indices.next() {
        let start_mult = &input[(i + START_MULT.len())..];
        if let Some(end_i) = start_mult.find(")") {
            let inside_args = &start_mult[..end_i];

            let numbers = inside_args
                .split(",")
                .map(|s| s.parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>();

            if numbers.len() == 2 {
                result += numbers[0] * numbers[1];
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_test() {
        let result = process_part1(&INPUT1);
        assert_eq!(result, 161);
    }

    #[test]
    fn part2_test() {
        let result = process_part2(&INPUT2);
        assert_eq!(result, 48);
    }
}
