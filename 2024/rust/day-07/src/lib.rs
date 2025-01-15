pub fn process_part1(input: &str) -> u64 {
    let valid_equations: Vec<u64> = Vec::new();

    // check multiples first, if not, substract and check multiples again

    input.lines().for_each(|line| {
        let mut iter = line.split(": ");
        let result = iter.next().unwrap().parse::<u64>().unwrap();
        let operands: Vec<u64> = iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| {
                s.parse::<u64>().unwrap()
            })
            .collect();

        let mut expected_result: u64 = 0;
        for operand in &operands {
            if result % operand == 0 {
                result /= operand;
            } else {
                result -= operand;
            }
        }

        // check which opreands can divide result
        let valid_operands: Vec<&u64> = operands.iter().filter(|&x| result % x == 0).collect();

        println!("{:?}", valid_operands);
    });

    valid_equations.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_test() {
        let result = process_part1(INPUT);
        assert_eq!(result, 3749);
    }
}
