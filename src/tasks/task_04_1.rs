use crate::utils::file_reader;

fn split_lines_into_char_arrays(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn check_horizontal(char_arrays: &Vec<Vec<char>>, found_pattern: &Vec<char>) -> usize {
    let mut amount = 0;

    for line in char_arrays {
        for i in 0..=line.len() - found_pattern.len() {
            let slice = &line[i..i + found_pattern.len()];

            if slice == found_pattern.as_slice() {
                amount += 1;
            }

            if slice.iter().rev().cloned().collect::<Vec<_>>() == found_pattern.as_slice() {
                amount += 1;
            }
        }
    }
    amount
}
fn check_vertical(char_arrays: &Vec<Vec<char>>, found_pattern: &Vec<char>) -> usize {
    let mut amount = 0;

    for col in 0..char_arrays[0].len() {
        for row in 0..=char_arrays.len() - found_pattern.len() {
            let vertical: Vec<char> = (0..found_pattern.len())
                .map(|i| char_arrays[row + i][col])
                .collect();

            if vertical == *found_pattern {
                amount += 1;
            }

            if vertical.iter().rev().cloned().collect::<Vec<_>>() == *found_pattern {
                amount += 1;
            }
        }
    }

    amount
}

fn check_diagonals(char_arrays: &Vec<Vec<char>>, found_pattern: &Vec<char>) -> usize {
    let mut amount = 0;

    for row in 0..=char_arrays.len() - found_pattern.len() {
        for col in 0..=char_arrays[0].len() - found_pattern.len() {
            let diagonal: Vec<char> = (0..found_pattern.len())
                .map(|i| char_arrays[row + i][col + i])
                .collect();

            if diagonal == *found_pattern {
                amount += 1;
            }

            if diagonal.iter().rev().cloned().collect::<Vec<_>>() == *found_pattern {
                amount += 1;
            }
        }
    }

    for row in 0..=char_arrays.len() - found_pattern.len() {
        for col in (found_pattern.len() - 1)..char_arrays[0].len() {
            let diagonal: Vec<char> = (0..found_pattern.len())
                .map(|i| char_arrays[row + i][col - i])
                .collect();
            if diagonal == *found_pattern {
                amount += 1;
            }
            if diagonal.iter().rev().cloned().collect::<Vec<_>>() == *found_pattern {
                amount += 1;
            }
        }
    }

    amount
}

pub fn run() {
    let mut amount = 0;
    let found_pattern = vec!['X', 'M', 'A', 'S'];

    match file_reader::read_file_lines("./src/data/04.txt") {
        Ok(lines) => {
            let char_arrays = split_lines_into_char_arrays(lines);
            amount += check_horizontal(&char_arrays, &found_pattern);
            amount += check_vertical(&char_arrays, &found_pattern);
            amount += check_diagonals(&char_arrays, &found_pattern);
        }

        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("04_1 Amount: {}", amount);
}
