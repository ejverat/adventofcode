fn count_word_occurrences_no_spaces(text: &str, target_word: &str) -> u32 {
    let mut count = 0;
    let target_len = target_word.len();
    
    let mut i = 0;
    while i < text.len() {
        if text[i..].starts_with(target_word) {
            count += 1;
            i += target_len;
        } else {
            i += 1;
        }
    }

    count
}

pub fn process_part1(input: &str) -> u32 {
    const WORD: &str = "XMAS";
    const RWORD: &str = "SAMX";

    let mut total_words = 0;

    let lines: Vec<&str> = input.lines().collect();

    // count words
    for line in lines.iter() {
        let words = count_word_occurrences_no_spaces(line, WORD);
        let rev_words = count_word_occurrences_no_spaces(line, RWORD);
        total_words += words + rev_words;

        println!("Line: {} | Horiz: {} | HorizRev: {}", line, words, rev_words);
    }

    let num_rows: i32 = lines.len() as i32;
    let num_cols: i32 = lines[0].len() as i32;


    // Vertical lines
    for j in 0..num_cols {
        let mut vertical_str = String::new();
        let mut i = 0;

        while i < num_rows {
            vertical_str.push(lines[i as usize].chars().nth(j as usize).unwrap());
            i += 1;
        }
        let words = count_word_occurrences_no_spaces(&vertical_str, WORD);
        let rev_words = count_word_occurrences_no_spaces(&vertical_str, RWORD);
        total_words += words + rev_words;

        println!("Line: {} | Vert: {} | VertRev: {}", &vertical_str, words, rev_words);

    }

    // Collect diagonals into a vector of &str
    for k in 0..(num_rows as i32) + (num_cols as i32) - 1 {
        let mut diagonal_str = String::new();
        let mut i = if k < num_rows { k } else { num_rows - 1 };
        let mut j = if k < num_rows { 0 } else { k - num_rows + 1 };

        while i >= 0 && j < num_cols {
            diagonal_str.push(lines[i as usize].chars().nth(j as usize).unwrap());
            i -= 1;
            j += 1;
        }
        let words = count_word_occurrences_no_spaces(&diagonal_str, WORD);
        let rev_words = count_word_occurrences_no_spaces(&diagonal_str, RWORD);
        total_words += words + rev_words;

        println!("Line: {} | Diag: {} | DiagRev: {}", &diagonal_str, words, rev_words);

    }

    for k in 0..(num_rows as i32) + (num_cols as i32) - 1 {
        let mut diagonal_str = String::new();
        let mut i = if k < num_rows {
            num_rows - 1
        } else {
            (num_rows - 1) - (k - (num_rows - 1))
        };
        let mut j = if k < num_rows { k } else { num_rows - 1 };

        while i >= 0 && j >= 0 {
            diagonal_str.push(lines[i as usize].chars().nth(j as usize).unwrap());
            i -= 1;
            j -= 1;
        }

        let words = count_word_occurrences_no_spaces(&diagonal_str, WORD);
        let rev_words = count_word_occurrences_no_spaces(&diagonal_str, RWORD);
        total_words += words + rev_words;

        println!("Line: {} | Diag: {} | DiagRev: {}", &diagonal_str, words, rev_words);
    }

    // Print the diagonals stored in the vector
    // for (idx, &ref diagonal) in diagonals.iter().enumerate() {
    //     println!("Diagonal {}: {}", idx + 1, diagonal);
    // }

    // count words in diagonals top left
    // for line in &diagonals {
    //     total_words += count_word_occurrences_no_spaces(line, RWORD);
    // }

    // // count words in reverse diagonals top left
    // for line in diagonals.iter().rev() {
    //     if line.contains(WORD) {
    //         total_words += 1;
    //     }
    // }

    // Collect diagonals into a vector of &str starting from the bottom right

    total_words
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn count_words_test() {
        const INPUT_TEXT: &str = "applebananaorangeapplegrapeappleapple";

        let result = count_word_occurrences_no_spaces(INPUT_TEXT, "apple");
        assert_eq!(result, 4);
    }

    #[test]
    fn part1_test() {
        let result = process_part1(INPUT);
        assert_eq!(result, 18);
    }
}
