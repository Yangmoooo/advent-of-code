use aoc2020::{print_answer, read_file};

fn main() {
    let input = read_file("aoc2020/inputs/day02.txt");
    print_answer(2, false, solve1(&input));
    print_answer(2, true, solve2(&input));
}

fn solve1(input: &str) -> Option<u32> {
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

fn solve2(input: &str) -> Option<u32> {
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
mod day02 {
    use super::*;
    const EXAMPLE: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE), Some(2));
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE), Some(1));
    }
}
