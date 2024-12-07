use std::fs;
use std::io;

pub fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}
