use std::cmp::Ordering;
use std::collections::HashSet;

use aoc2020::*;

fn main() {
    let input = read_file("aoc2020/inputs/day01.txt");
    let answer_a = solve_a(&input);
    let answer_b = solve_b(&input);
    print_answer(1, (answer_a, answer_b));
}

fn solve_a(input: &str) -> Option<usize> {
    let nums = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

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

fn solve_b(input: &str) -> Option<usize> {
    let mut nums = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

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
mod day_01 {
    use super::*;
    const EXAMPLE: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn part_a() {
        assert_eq!(solve_a(EXAMPLE), Some(514579));
    }

    #[test]
    fn part_b() {
        assert_eq!(solve_b(EXAMPLE), Some(241861950));
    }
}
