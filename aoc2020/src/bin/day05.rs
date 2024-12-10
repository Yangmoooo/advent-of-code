use aoc2020::{print_answer, read_file};

fn main() {
    let input = read_file("aoc2020/inputs/day05.txt");
    print_answer(5, false, solve1(&input));
    print_answer(5, true, solve2(&input));
}

fn solve1(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|line| {
            let binary_stream = line
                .chars()
                .map(|c| match c {
                    'B' | 'R' => '1',
                    _ => '0',
                })
                .collect::<String>();
            usize::from_str_radix(&binary_stream, 2).unwrap()
        })
        .max()
}

fn solve2(input: &str) -> Option<usize> {
    let mut seat_ids: Vec<usize> = input
        .lines()
        .map(|line| {
            let binary_stream = line
                .chars()
                .map(|c| match c {
                    'B' | 'R' => '1',
                    _ => '0',
                })
                .collect::<String>();
            usize::from_str_radix(&binary_stream, 2).unwrap()
        })
        .collect();
    seat_ids.sort_unstable();
    seat_ids
        .windows(2)
        .find(|&pair| pair[1] - pair[0] == 2)
        .map(|pair| pair[0] + 1)
}

#[cfg(test)]
mod day05 {
    use super::*;

    #[test]
    fn part1() {
        let example = "FBFBBFFRLR\n\
                       BFFFBBFRRR\n\
                       FFFBBBFRRR\n\
                       BBFFBBFRLL";
        assert_eq!(solve1(example), Some(820));
    }
}
