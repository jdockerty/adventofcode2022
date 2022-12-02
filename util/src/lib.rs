use std::fs;

pub fn read_input(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("could not open file");
    contents
}

