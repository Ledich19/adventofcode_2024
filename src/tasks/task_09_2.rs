use crate::utils::file_reader;

fn create_map(number: &str) -> Vec<String> {
    let char_arr: Vec<char> = number.trim().chars().collect();
    let mut result = Vec::new();
    let mut id = 0;

    char_arr.iter().enumerate().for_each(|(i, char_num)| {
        let num = char_num.to_digit(10).unwrap_or(0) as usize;

        if i % 2 == 0 {
            for _ in 0..num {
                result.push(format!("{}", id));
            }
            id += 1;
        } else {
            for _ in 0..num {
                result.push(".".to_string());
            }
        }
    });

    result
}

fn group_and_count(input: &Vec<String>) -> Vec<(String, usize)> {
    let mut result = Vec::new();
    let mut count = 1;
    for i in 1..input.len() {
        if input[i] == input[i - 1] {
            count += 1;
        } else {
            result.push((input[i - 1].clone(), count));
            count = 1;
        }
    }
    result.push((input[input.len() - 1].clone(), count));
    result
}
fn reverse_group_and_count(input: &Vec<(String, usize)>) -> Vec<String> {
    let mut result = Vec::new();

    for (value, count) in input {
        for _ in 0..*count {
            result.push(value.clone());
        }
    }

    result
}

fn defragmentation_data(map: &Vec<String>) -> Vec<String> {
    let mut work_array = group_and_count(map);

    let mut i = work_array.len() - 1;
    while i > 0 {
        let (work_num, work_amount) = work_array[i].clone();
        if work_num != "." {
            let mut j = 0;
            while j < work_array.len() {
                let (current_num, current_amount) = work_array[j].clone();

                if current_num == "." && (current_amount == work_amount) && (i > j) {
                    work_array[i] = (current_num, current_amount);
                    work_array[j] = (work_num, work_amount);

                    break;
                }

                if current_num == "." && (current_amount > work_amount) && (i > j) {
                    work_array[i] = (current_num, work_amount);
                    work_array[j] = (work_num, work_amount);
                    work_array.insert(j + 1, (".".to_string(), current_amount - work_amount));
                    i += 1;

                    break;
                }

                j += 1;
            }
        }
        i -= 1;
    }

    reverse_group_and_count(&work_array)
}

fn calculate_sum(defragmented_data: &Vec<String>) -> u64 {
    let mut hash: u64 = 0;

    for (i, item) in defragmented_data.iter().enumerate() {
        if let Ok(char_num) = item.parse::<u32>() {
            hash += i as u64 * char_num as u64;
        }
    }

    hash
}

#[allow(dead_code)]
pub fn run() {
    match file_reader::read_file_to_string("./src/data/09.txt") {
        Ok(data) => {
            let data_map = create_map(&data);
            let defragmented_data = defragmentation_data(&data_map);
            let sum = calculate_sum(&defragmented_data);
            println!("09_2 Sum of indices: {}", sum);
        }
        Err(e) => eprintln!("ERROR: {}", e),
    }
}
