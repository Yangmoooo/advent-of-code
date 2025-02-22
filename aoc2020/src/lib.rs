pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("Failed to read file")
}

pub fn print_answer(day: u8, answers: (Option<usize>, Option<usize>)) {
    println!("[AoC 2020 {day:02}-A] {:?}", answers.0);
    println!("[AoC 2020 {day:02}-B] {:?}", answers.1);
}
