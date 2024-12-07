use crate::utils::file_reader;
use regex::Regex;

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

pub fn run() {
    let mut result = 0;

    match file_reader::read_file_to_string("./src/data/03.txt") {
        Ok(content) => {
            let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
            for capture in re.captures_iter(&content) {
                let match_str = capture.get(0).unwrap().as_str();
                result += do_multiple(match_str);
            }
        }
        Err(e) => println!("ERROR: {}", e),
    }
    println!("03_1 Sum all multiple: {}", result);
}
