pub fn process_part1(input: &str) -> String {
    let mut l = Vec::new();
    let mut r = Vec::new();
    input.lines().for_each(|side_side_ids| {
        let mut ints = Vec::new();
        side_side_ids.split_whitespace().for_each(|item| ints.push(item.parse::<i32>().unwrap()));
        l.push(ints[0]);
        r.push(ints[1]);
    });

    l.sort();
    r.sort();

    let result = l.iter().zip(r.iter()).fold(0, |acc, (l, r)| acc + (l - r).abs()).to_string();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = process_part1(&input);
        assert_eq!(result, "11");
    }
}