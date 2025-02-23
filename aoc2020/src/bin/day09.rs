use aoc2020::*;

fn main() {
    let input = read_file("aoc2020/inputs/day09.txt");
    let answer_a = solve_a(&input);
    let answer_b = solve_b(&input);
    print_answer(9, (answer_a, answer_b));
}

fn solve_a(input: &str) -> Option<usize> {
    let numbers: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    let size = 25;
    for window in numbers.windows(size + 1) {
        let target = window[size];
        let mut found = false;
        for i in 0..size {
            for j in i + 1..size {
                if window[i] + window[j] == target {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            return Some(target);
        }
    }
    None
}

fn solve_b(input: &str) -> Option<usize> {
    let target = solve_a(input).unwrap();
    let numbers: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    let (mut sum, mut left, mut right) = (0, 0, 0);
    while right < numbers.len() {
        sum += numbers[right];
        right += 1;

        while sum > target && left < right {
            sum -= numbers[left];
            left += 1;
        }

        if sum == target {
            let window = &numbers[left..right];
            return Some(window.iter().min().unwrap() + window.iter().max().unwrap());
        }
    }

    None
}
