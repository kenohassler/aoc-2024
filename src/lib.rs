use std::{fs, io};

const IN_DIR: &str = "inputs";

pub fn example(day: u8) -> io::Result<String> {
    fs::read_to_string(format!("{IN_DIR}/day{day}_example.txt"))
}

pub fn input(day: u8) -> io::Result<String> {
    fs::read_to_string(format!("{IN_DIR}/day{day}.txt"))
}
