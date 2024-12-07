use crate::utils::file_reader;
use regex::Regex;
#[allow(dead_code)]
fn do_multiple(data: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    if let Some(captures) = re.captures(&data) {
        let num1: i32 = captures[1].parse().unwrap();
        let num2: i32 = captures[2].parse().unwrap();
        return num1 * num2;
    } else {
        return 0;
    }
}
#[allow(dead_code)]
fn process_file_content(content: &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don\'t\(\)").unwrap();
    let mut result = 0;
    let mut skip_next = false;

    for capture in re.captures_iter(content) {
        let matched_str = capture.get(0).unwrap().as_str();

        if matched_str == "don't()" {
            skip_next = true;
        } else if matched_str == "do()" {
            skip_next = false;
        } else {
            if !skip_next {
                result += do_multiple(matched_str);
            }
        }
    }

    result
}
#[allow(dead_code)]
pub fn run() {
    match file_reader::read_file_to_string("./src/data/03.txt") {
        Ok(content) => {
            let result = process_file_content(&content);
            println!("03_2 Sum all multiple: {}", result);
        }
        Err(e) => eprintln!("ERROR: {}", e),
    }
}
