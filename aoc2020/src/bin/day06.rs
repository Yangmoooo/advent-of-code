use std::collections::HashSet;

use aoc2020::{print_answer, read_file};

fn main() {
    let input = read_file("aoc2020/inputs/day06.txt");
    print_answer(6, false, solve1(&input));
    print_answer(6, true, solve2(&input));
}

fn solve1(input: &str) -> Option<usize> {
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

fn solve2(input: &str) -> Option<usize> {
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
mod day06 {
    use super::*;

    #[test]
    fn part1() {
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
        assert_eq!(solve1(example), Some(11));
    }

    #[test]
    fn part2() {
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
        assert_eq!(solve2(example), Some(6));
    }
}
