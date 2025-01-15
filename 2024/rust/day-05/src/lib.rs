use std::collections::HashMap;

pub fn process_part1(input: &str) -> u64 {
    // read input, split in new lines and stop until there is a blank line
    let mut input_splitted = input.split("\n\n");
    // println!("{:?}", input_splitted.next().unwrap());
    // println!("{:?}", input_splitted.next().unwrap());

    // create a map of rules
    let mut rules: HashMap<u64, Vec<Vec<u64>>> = HashMap::new();
    input_splitted
        .next()
        .unwrap()
        .lines()
        .for_each(|rule_text| {
            let mut rule = Vec::new();
            rule_text.split("|").for_each(|page| {
                let page_num = page.parse::<u64>().unwrap();
                rule.push(page_num)
            });
            for page_num in &rule {
                if rules.contains_key(page_num) {
                    rules.get_mut(page_num).unwrap().push(rule.clone());
                } else {
                    rules.insert(*page_num, vec![rule.clone()]);
                }
            }
        });

    // create a vector of pages
    let mut pages_pruduced: Vec<Vec<u64>> = Vec::new();

    input_splitted
        .next()
        .unwrap()
        .lines()
        .for_each(|update_pages| {
            let mut pages = Vec::new();

            update_pages.split(",").for_each(|page_text| {
                pages.push(page_text.parse::<u64>().unwrap());
            });

            pages_pruduced.push(pages);
        });

    // collect produced pages are correct based on rules
    let mut correct_pages: Vec<u64> = Vec::new();
    let mut middle_sum: u64 = 0;

    for input_pages in pages_pruduced {
        let mut correct = true;
        let mut middle: u64 = 0;
        for i in 0..input_pages.len() {
            if i == input_pages.len() / 2 {
                middle = input_pages[i];
            }
            // for each rule in rules, get vectors that match with current input_page
            rules
                .get(&input_pages[i])
                .unwrap()
                .into_iter()
                .for_each(|rule_page| {
                    if correct {
                        // check if page accomplish with rule
                        if input_pages[i] == rule_page[0] {
                            // check if no rule_pages[1] on input_pages before index i
                            for j in 0..i {
                                if input_pages[j] == rule_page[1] {
                                    println!(
                                        "{:?} found {} on index {}",
                                        input_pages, rule_page[1], j
                                    );
                                    correct = false;
                                    break;
                                }
                            }
                        } else if input_pages[i] == rule_page[1] {
                            // check if no rule_pages[0] on input_pages after index i
                            for j in i + 1..input_pages.len() {
                                if input_pages[j] == rule_page[0] {
                                    println!(
                                        "{:?} found {} on index {}",
                                        input_pages, rule_page[0], j
                                    );
                                    correct = false;
                                    break;
                                }
                            }
                        }
                    }
                });
        }

        if correct {
            correct_pages.push(input_pages[0]);
            middle_sum += middle;
        }
    }

    middle_sum
}

pub fn process_part2(input: &str) -> u64 {
    // read input, split in new lines and stop until there is a blank line
    let mut input_splitted = input.split("\n\n");
    // println!("{:?}", input_splitted.next().unwrap());
    // println!("{:?}", input_splitted.next().unwrap());

    // create a map of rules
    let mut rules: HashMap<u64, Vec<Vec<u64>>> = HashMap::new();
    input_splitted
        .next()
        .unwrap()
        .lines()
        .for_each(|rule_text| {
            let mut rule = Vec::new();
            rule_text.split("|").for_each(|page| {
                let page_num = page.parse::<u64>().unwrap();
                rule.push(page_num)
            });
            for page_num in &rule {
                if rules.contains_key(page_num) {
                    rules.get_mut(page_num).unwrap().push(rule.clone());
                } else {
                    rules.insert(*page_num, vec![rule.clone()]);
                }
            }
        });

    // create a vector of pages
    let mut pages_pruduced: Vec<Vec<u64>> = Vec::new();

    input_splitted
        .next()
        .unwrap()
        .lines()
        .for_each(|update_pages| {
            let mut pages = Vec::new();

            update_pages.split(",").for_each(|page_text| {
                pages.push(page_text.parse::<u64>().unwrap());
            });

            pages_pruduced.push(pages);
        });

    // collect produced pages are correct based on rules
    let mut middle_sum: u64 = 0;

    for mut input_pages in pages_pruduced {
        println!("input_pages: {:?}", input_pages);
        let mut correct = false;
        let mut corrected_page = false;
        while correct == false {
            correct = true;
            for i in 0..input_pages.len() {
                // for each rule in rules, get vectors that match with current input_page
                rules
                    .get(&input_pages[i])
                    .unwrap()
                    .into_iter()
                    .for_each(|rule_page| {
                        // check if page accomplish with rule
                        if input_pages[i] == rule_page[0] {
                            // check if no rule_pages[1] on input_pages before index i
                            for j in 0..i {
                                if input_pages[j] == rule_page[1] {
                                    println!(
                                        "{:?} found {} on index {}",
                                        input_pages, rule_page[1], j
                                    );
                                    correct = false;
                                    corrected_page = true;
                                    // move input_pages[i] to left until index j
                                    for k in i - 1..j {
                                        input_pages.swap(k, k + 1);
                                    }
                                    break;
                                }
                            }
                        } else if input_pages[i] == rule_page[1] {
                            // check if no rule_pages[0] on input_pages after index i
                            for j in i + 1..input_pages.len() {
                                if input_pages[j] == rule_page[0] {
                                    println!(
                                        "{:?} found {} on index {}",
                                        input_pages, rule_page[0], j
                                    );
                                    correct = false;
                                    corrected_page = true;

                                    // rotate input_pages[i] to right until index j
                                    for k in i..j {
                                        input_pages.swap(k, k + 1);
                                    }
                                    break;
                                }
                            }
                        }
                    });
            }
        }

        if corrected_page {
            let i = input_pages.len() / 2;
            let middle = input_pages[i];
            middle_sum += middle;
            println!("{:?}", input_pages);
        }
    }

    middle_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_test() {
        let result = process_part1(INPUT);
        assert_eq!(result, 143);
    }

    #[test]
    fn part2_test() {
        let result = process_part2(INPUT);
        assert_eq!(result, 123);
    }
}
