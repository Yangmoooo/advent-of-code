use aoc2020::*;

fn main() {
    let input = read_file("aoc2020/inputs/day10.txt");
    let answer_a = solve_a(&input);
    let answer_b = solve_b(&input);
    print_answer(10, (answer_a, answer_b));
}

fn solve_a(input: &str) -> Option<usize> {
    let mut jolts: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    jolts.sort_unstable();
    jolts.insert(0, 0);
    let (mut one, mut three) = (0, 1);
    for jolt in jolts.windows(2) {
        match jolt[1] - jolt[0] {
            1 => one += 1,
            3 => three += 1,
            _ => return None,
        }
    }
    Some(one * three)
}

fn solve_b(input: &str) -> Option<usize> {
    let mut jolts: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    jolts.sort_unstable();
    jolts.insert(0, 0);
    let mut ways = vec![0; jolts.len()];
    ways[0] = 1;
    for (i, jolt) in jolts.iter().enumerate() {
        for (j, next) in jolts.iter().enumerate().skip(i + 1).take(3) {
            if next - jolt <= 3 {
                ways[j] += ways[i];
            }
        }
    }
    Some(ways.last().copied().unwrap())
}

#[cfg(test)]
mod day_10 {
    use super::*;

    #[test]
    fn part_a() {
        let example = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        assert_eq!(solve_a(example), Some(35));
    }

    #[test]
    fn part_b() {
        let example = "28\n33\n18\n42\n31\n\
                       14\n46\n20\n48\n47\n\
                       24\n23\n49\n45\n19\n\
                       38\n39\n11\n1\n32\n\
                       25\n35\n8\n17\n7\n\
                       9\n4\n2\n34\n10\n3";
        assert_eq!(solve_b(example), Some(19208));
    }
}
