use crate::utils::file_reader;

fn process_file_content(content: Vec<String>) -> Vec<(u64, Vec<u64>)> {
    let mut result = Vec::new();

    for item in content.iter() {
        let get_value: Vec<&str> = item.split(':').collect();
        let test_value = get_value[0].parse::<u64>().unwrap();
        let get_operators = get_value[1]
            .trim()
            .split(' ')
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect();
        result.push((test_value, get_operators));
    }

    result
}

fn combine_numbers(a: u64, b: u64) -> u64 {
    let result = format!("{}{}", a, b);
    result.parse::<u64>().unwrap()
}

fn get_variants(result: u64, numbers: &[u64]) -> Vec<u64> {
    let mut result_vec: Vec<u64> = Vec::new();

    if numbers.len() == 0 {
        result_vec.push(result);
        return result_vec;
    }

    let sum = result + numbers[0];
    let mul = result * numbers[0];
    let conc = combine_numbers(result, numbers[0]);

    result_vec.extend(get_variants(sum, &numbers[1..]));
    result_vec.extend(get_variants(mul, &numbers[1..]));
    result_vec.extend(get_variants(conc, &numbers[1..]));

    result_vec
}

#[allow(dead_code)]
pub fn run() {
    let mut result_sum: u64 = 0;
    match file_reader::read_file_lines("./src/data/07.txt") {
        Ok(content) => {
            let result = process_file_content(content);

            for item in result {
                let (test_value, operators) = item;
                let variants = get_variants(0, &operators);
                if variants.contains(&test_value) {
                    result_sum += test_value;
                }
            }

            println!("07_2 Sum all  {:?}", result_sum);
        }
        Err(e) => eprintln!("ERROR: {}", e),
    }
}
