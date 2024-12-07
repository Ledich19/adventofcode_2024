use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}

pub fn read_file_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let path = Path::new(file_path);
    let file = fs::File::open(path)?;

    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => lines.push(content),
            Err(e) => return Err(e),
        }
    }

    Ok(lines)
}
