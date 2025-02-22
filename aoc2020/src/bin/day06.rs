use std::collections::HashSet;

use aoc2020::*;

fn main() {
    let input = read_file("aoc2020/inputs/day06.txt");
    let answer_a = solve_a(&input);
    let answer_b = solve_b(&input);
    print_answer(6, (answer_a, answer_b));
}

fn solve_a(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|group| {
                group
                    .chars()
                    .filter(|&c| c.is_ascii_alphabetic())
                    .collect::<HashSet<char>>()
                    .len()
            })
            .sum(),
    )
}

fn solve_b(input: &str) -> Option<usize> {
    let groups = input.split("\n\n").collect::<Vec<&str>>();
    let mut cnt = 0;
    for group in groups {
        let mut answer = ('a'..='z').collect::<HashSet<char>>();
        for person in group.lines() {
            let ans = person.chars().collect::<HashSet<char>>();
            answer = answer.intersection(&ans).copied().collect();
        }
        cnt += answer.len();
    }
    Some(cnt)
}

#[cfg(test)]
mod day_06 {
    use super::*;

    #[test]
    fn part_a() {
        let example = "abc\n\
                       \n\
                       a\n\
                       b\n\
                       c\n\
                       \n\
                       ab\n\
                       ac\n\
                       \n\
                       a\n\
                       a\n\
                       a\n\
                       a\n\
                       \n\
                       b";
        assert_eq!(solve_a(example), Some(11));
    }

    #[test]
    fn part_b() {
        let example = "abc\n\
                       \n\
                       a\n\
                       b\n\
                       c\n\
                       \n\
                       ab\n\
                       ac\n\
                       \n\
                       a\n\
                       a\n\
                       a\n\
                       a\n\
                       \n\
                       b";
        assert_eq!(solve_b(example), Some(6));
    }
}
