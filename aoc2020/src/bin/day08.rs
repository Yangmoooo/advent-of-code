use aoc2020::*;

fn main() {
    let input = read_file("aoc2020/inputs/day08.txt");
    let answer_a = solve_a(&input);
    let answer_b = solve_b(&input);
    print_answer(8, (answer_a, answer_b));
}

fn solve_a(input: &str) -> Option<usize> {
    let instructions: Vec<(&str, i32)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let op = parts.next().unwrap();
            let arg = parts.next().unwrap().parse().unwrap();
            (op, arg)
        })
        .collect();

    let mut visited = vec![false; instructions.len()];
    let mut acc = 0;
    let mut index = 0;
    while !visited[index] {
        visited[index] = true;
        let (op, arg) = instructions[index];
        match op {
            "acc" => {
                acc += arg;
                index += 1;
            }
            "jmp" => {
                index = (index as i32 + arg) as usize;
            }
            "nop" => {
                index += 1;
            }
            _ => unreachable!(),
        }
    }
    Some(acc as usize)
}

fn solve_b(input: &str) -> Option<usize> {
    let instructions: Vec<(&str, i32)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let op = parts.next().unwrap();
            let arg = parts.next().unwrap().parse().unwrap();
            (op, arg)
        })
        .collect();

    for i in 0..instructions.len() {
        let mut ins = instructions.clone();
        match ins[i].0 {
            "jmp" => ins[i].0 = "nop",
            "nop" => ins[i].0 = "jmp",
            _ => continue,
        }

        let mut visited = vec![false; ins.len()];
        let mut acc = 0;
        let mut index = 0;
        while index < ins.len() && !visited[index] {
            visited[index] = true;
            let (op, arg) = ins[index];
            match op {
                "acc" => {
                    acc += arg;
                    index += 1;
                }
                "jmp" => {
                    index = (index as i32 + arg) as usize;
                }
                "nop" => {
                    index += 1;
                }
                _ => unreachable!(),
            }
        }
        if index == ins.len() {
            return Some(acc as usize);
        }
    }
    None
}

#[cfg(test)]
mod day_08 {
    use super::*;

    #[test]
    fn part_a() {
        let example = "nop +0\n\
                       acc +1\n\
                       jmp +4\n\
                       acc +3\n\
                       jmp -3\n\
                       acc -99\n\
                       acc +1\n\
                       jmp -4\n\
                       acc +6";
        assert_eq!(solve_a(example), Some(5));
    }

    #[test]
    fn part_b() {
        let example = "nop +0\n\
                       acc +1\n\
                       jmp +4\n\
                       acc +3\n\
                       jmp -3\n\
                       acc -99\n\
                       acc +1\n\
                       jmp -4\n\
                       acc +6";
        assert_eq!(solve_b(example), Some(8));
    }
}
