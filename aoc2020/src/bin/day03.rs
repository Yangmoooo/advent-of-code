use aoc2020::*;

fn main() {
    let input = read_file("aoc2020/inputs/day03.txt");
    let answer_a = solve_a(&input);
    let answer_b = solve_b(&input);
    print_answer(3, (answer_a, answer_b));
}

fn solve_a(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < lines.len() {
        let line = lines[y];
        let c = line.chars().nth(x % line.len()).unwrap();
        if c == '#' {
            trees += 1;
        }
        x += 3;
        y += 1;
    }
    Some(trees)
}

fn solve_b(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut trees = 1;
    for (dx, dy) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut x = 0;
        let mut y = 0;
        let mut count = 0;
        while y < lines.len() {
            let line = lines[y];
            let c = line.chars().nth(x % line.len()).unwrap();
            if c == '#' {
                count += 1;
            }
            x += dx;
            y += dy;
        }
        trees *= count;
    }
    Some(trees)
}

#[cfg(test)]
mod day_03 {
    use super::*;
    const EXAMPLE: &str = "..##.......\n\
                           #...#...#..\n\
                           .#....#..#.\n\
                           ..#.#...#.#\n\
                           .#...##..#.\n\
                           ..#.##.....\n\
                           .#.#.#....#\n\
                           .#........#\n\
                           #.##...#...\n\
                           #...##....#\n\
                           .#..#...#.#";

    #[test]
    fn part_a() {
        assert_eq!(solve_a(EXAMPLE), Some(7));
    }

    #[test]
    fn part_b() {
        assert_eq!(solve_b(EXAMPLE), Some(336));
    }
}
