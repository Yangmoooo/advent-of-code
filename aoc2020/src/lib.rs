use std::fs::File;
use std::io::Read;

pub fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    input
}

pub fn print_answer(day: &str, part: &str, answer: Option<u32>) {
    match answer {
        Some(answer) => println!("[AoC 2020 {day} {part}] {answer:?}"),
        None => println!("[AoC 2020 {day} {part}] No answer found"),
    }
}
