use std::fs::File;
use std::io::Read;

pub fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    input
}

pub fn print_answer(day: u8, is_second: bool, answer: Option<u32>) {
    let part = if is_second { "Part2" } else { "Part1" };
    match answer {
        Some(answer) => println!("[AoC 2020 Day{day:02} {part}] {answer}"),
        None => println!("[AoC 2020 Day{day:02} {part}] No answer found"),
    }
}
