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

fn defragmentation_data(map: &Vec<String>) -> Vec<String> {
    let mut result = map.clone();

    for i in 0..result.len() - 1 {
        if result[i] == "." {
            let char_at_end_i = result.iter().rposition(|x| x != ".").unwrap();
            if char_at_end_i <= i {
                break;
            }
            result[i] = result[char_at_end_i].clone();
            result[char_at_end_i] = ".".to_string();
        }
    }

    result
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
            println!("09_1 control sum: {}", sum);
        }
        Err(e) => eprintln!("ERROR: {}", e),
    }
}
