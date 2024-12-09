use std::cmp::Ordering;
use std::collections::HashSet;

use aoc2020::{print_answer, read_file};

fn main() {
    let input = read_file("aoc2020/inputs/day01.txt");
    print_answer("day01", "part1", part1(&input));
    print_answer("day01", "part2", part2(&input));
}

fn part1(input: &str) -> Option<u32> {
    let nums = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut done = HashSet::new();
    for i in nums {
        let pair = 2020 - i;
        if done.contains(&pair) {
            return Some(i * pair);
        }
        done.insert(i);
    }
    None
}

fn part2(input: &str) -> Option<u32> {
    let mut nums = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    nums.sort();
    let n = nums.len();

    for i in 0..n {
        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            match sum.cmp(&2020) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => return Some(nums[i] * nums[left] * nums[right]),
            }
        }
    }
    None
}

#[cfg(test)]
mod day01 {
    const EXAMPLE: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn part1() {
        assert_eq!(super::part1(EXAMPLE), Some(514579));
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(EXAMPLE), Some(241861950));
    }
}
