use aoc2020::*;

fn main() {
    let input = read_file("aoc2020/inputs/day02.txt");
    let answer_a = solve_a(&input);
    let answer_b = solve_b(&input);
    print_answer(2, (answer_a, answer_b));
}

fn solve_a(input: &str) -> Option<usize> {
    let entries = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let mut range = parts.next().unwrap().split('-');
            let min = range.next().unwrap().parse::<u32>().unwrap();
            let max = range.next().unwrap().parse::<u32>().unwrap();
            let letter = parts.next().unwrap().chars().next().unwrap();
            let password = parts.next().unwrap();
            (min, max, letter, password)
        })
        .collect::<Vec<(u32, u32, char, &str)>>();

    let mut valid = 0;
    for (min, max, letter, password) in entries {
        let count = password.chars().filter(|&c| c == letter).count() as u32;
        if count >= min && count <= max {
            valid += 1;
        }
    }
    Some(valid)
}

fn solve_b(input: &str) -> Option<usize> {
    let entries = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let mut range = parts.next().unwrap().split('-');
            let pos1 = range.next().unwrap().parse::<usize>().unwrap();
            let pos2 = range.next().unwrap().parse::<usize>().unwrap();
            let letter = parts.next().unwrap().chars().next().unwrap();
            let password = parts.next().unwrap();
            (pos1, pos2, letter, password)
        })
        .collect::<Vec<(usize, usize, char, &str)>>();

    let mut valid = 0;
    for (pos1, pos2, letter, password) in entries {
        let chars: Vec<char> = password.chars().collect();
        if (chars[pos1 - 1] == letter) ^ (chars[pos2 - 1] == letter) {
            valid += 1;
        }
    }
    Some(valid)
}

#[cfg(test)]
mod day_02 {
    use super::*;
    const EXAMPLE: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    #[test]
    fn part_a() {
        assert_eq!(solve_a(EXAMPLE), Some(2));
    }

    #[test]
    fn part_b() {
        assert_eq!(solve_b(EXAMPLE), Some(1));
    }
}
