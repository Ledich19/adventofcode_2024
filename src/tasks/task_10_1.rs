use crate::utils::file_reader;

fn array_of_chat_to_number(chars: &Vec<Vec<char>>) -> Vec<Vec<i8>> {
    chars
        .iter()
        .map(|inner_vec| {
            inner_vec
                .iter()
                .map(|&c| {
                    if c.is_digit(10) {
                        c.to_digit(10).unwrap() as i8
                    } else {
                        -1
                    }
                })
                .collect()
        })
        .collect()
}

fn find_start(numbers: &Vec<Vec<i8>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for i in 0..numbers.len() {
        for j in 0..numbers[i].len() {
            if numbers[i][j] == 0 {
                result.push((i, j));
            }
        }
    }
    result
}

fn find_paths_for_one(
    numbers: &Vec<Vec<i8>>,
    i: usize,
    j: usize,
    prev: i8,
    visited: &mut Vec<Vec<usize>>,
) -> usize {
    let rows = numbers.len();
    let cols = numbers[0].len();

    if (numbers[i][j] == 9) && (prev == 8) && visited[i][j] != 1 {
        visited[i][j] = 1;
        return 1;
    }

    let mut result = 0;

    if j + 1 < cols && numbers[i][j] + 1 == numbers[i][j + 1] {
        result += find_paths_for_one(numbers, i, j + 1, numbers[i][j], visited);
    }
    if i + 1 < rows && numbers[i][j] + 1 == numbers[i + 1][j] {
        result += find_paths_for_one(numbers, i + 1, j, numbers[i][j], visited);
    }
    if i > 0 && numbers[i][j] + 1 == numbers[i - 1][j] {
        result += find_paths_for_one(numbers, i - 1, j, numbers[i][j], visited);
    }
    if j > 0 && numbers[i][j] + 1 == numbers[i][j - 1] {
        result += find_paths_for_one(numbers, i, j - 1, numbers[i][j], visited);
    }

    result
}

fn count_all_paths(numbers: &Vec<Vec<i8>>, starts: Vec<(usize, usize)>) -> usize {
    let mut result = 0;

    for item in starts {
        let (i, j) = item;
        let mut visited = vec![vec![0; numbers[0].len()]; numbers.len()];
        let found_paths = find_paths_for_one(&numbers, i, j, 0, &mut visited);
        result += found_paths;
    }

    result
}

#[allow(dead_code)]
pub fn run() {
    match file_reader::read_file_to_array_of_arrays_by_char("./src/data/10.txt") {
        Ok(data) => {
            let numbers = array_of_chat_to_number(&data);
            let starts = find_start(&numbers);
            let result = count_all_paths(&numbers, starts);
            println!("10_1 Sum all paths: {}", result);
        }
        Err(e) => eprintln!("ERROR: {}", e),
    }
}
