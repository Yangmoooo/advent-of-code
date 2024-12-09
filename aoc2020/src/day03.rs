use aoc2020::{print_answer, read_file};

fn main() {
    let input = read_file("aoc2020/inputs/day03.txt");
    print_answer("day03", "part1", part1(&input));
    print_answer("day03", "part2", part2(&input));
}

fn part1(input: &str) -> Option<u32> {
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

fn part2(input: &str) -> Option<u32> {
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
mod day03 {
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
    fn part1() {
        assert_eq!(super::part1(EXAMPLE), Some(7));
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(EXAMPLE), Some(336));
    }
}
