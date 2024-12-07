use crate::utils::file_reader;

fn split_lines_into_char_arrays(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn check_x_mas(char_arrays: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    if row == 0 || row == char_arrays.len() - 1 || col == 0 || col == char_arrays[row].len() - 1 {
        return 0;
    }

    let l_t = char_arrays[row - 1][col - 1];
    let l_b = char_arrays[row + 1][col - 1];
    let r_t = char_arrays[row - 1][col + 1];
    let r_b = char_arrays[row + 1][col + 1];

    if l_t == r_b || l_b == r_t {
        return 0;
    }
    if (l_t != 'M' && l_t != 'S')
        || (l_b != 'M' && l_b != 'S')
        || (r_b != 'M' && r_b != 'S')
        || (r_t != 'M' && r_t != 'S')
    {
        return 0;
    }

    1
}
#[allow(dead_code)]
pub fn run() {
    let mut amount = 0;

    match file_reader::read_file_lines("./src/data/04.txt") {
        Ok(lines) => {
            let char_arrays = split_lines_into_char_arrays(lines);
            for i in 0..=char_arrays.len() - 1 {
                for j in 0..=char_arrays[i].len() - 1 {
                    if char_arrays[i][j] == 'A' {
                        amount += check_x_mas(&char_arrays, i, j)
                    }
                }
            }
        }

        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("04_2 Amount: {}", amount);
}
