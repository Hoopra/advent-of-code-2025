use std::fs::read_to_string;

pub fn read_input(file_path: &str) -> String {
    let text = read_to_string(file_path).unwrap();

    text
}