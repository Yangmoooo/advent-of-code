use std::collections::HashMap;

use aoc2020::*;

fn main() {
    let input = read_file("aoc2020/inputs/day04.txt");
    let answer_a = solve_a(&input);
    let answer_b = solve_b(&input);
    print_answer(4, (answer_a, answer_b));
}

fn solve_a(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .filter(|passport| {
                let fields = passport
                    .split_whitespace()
                    .map(|field| field.split(':').next().unwrap());
                let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
                required_fields
                    .iter()
                    .all(|field| fields.clone().any(|f| f == *field))
            })
            .count(),
    )
}

fn solve_b(input: &str) -> Option<usize> {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    Some(
        input
            .split("\n\n")
            .map(parse_passport)
            .filter(|fields| {
                required_fields
                    .iter()
                    .all(|field| fields.contains_key(*field))
            })
            .filter(|fields| is_valid_passport(fields))
            .count(),
    )
}

fn parse_passport(passport: &str) -> HashMap<&str, &str> {
    passport
        .split_whitespace()
        .map(|field| {
            let mut parts = field.split(':');
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect()
}

fn is_valid_passport(fields: &HashMap<&str, &str>) -> bool {
    fields.iter().all(|(field, value)| match *field {
        "byr" => (1920..=2002).contains(&value.parse().unwrap()),
        "iyr" => (2010..=2020).contains(&value.parse().unwrap()),
        "eyr" => (2020..=2030).contains(&value.parse().unwrap()),
        "hgt" => {
            let (height, unit) = value.split_at(value.len() - 2);
            match unit {
                "cm" => (150..=193).contains(&height.parse().unwrap()),
                "in" => (59..=76).contains(&height.parse().unwrap()),
                _ => false,
            }
        }
        "hcl" => {
            value.len() == 7
                && value.starts_with('#')
                && value.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
        _ => true,
    })
}

#[cfg(test)]
mod day_04 {
    use super::*;

    #[test]
    fn part_a() {
        let example = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                       byr:1937 iyr:2017 cid:147 hgt:183cm\n\
                       \n\
                       iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                       hcl:#cfa07d byr:1929\n\
                       \n\
                       hcl:#ae17e1 iyr:2013\n\
                       eyr:2024\n\
                       ecl:brn pid:760753108 byr:1931\n\
                       hgt:179cm\n\
                       \n\
                       hcl:#cfa07d eyr:2025 pid:166559648\n\
                       iyr:2011 ecl:brn hgt:59in";
        assert_eq!(solve_a(example), Some(2));
    }

    #[test]
    fn part_b() {
        let example1 = "eyr:1972 cid:100\n\
                       hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
                       \n\
                       iyr:2019\n\
                       hcl:#602927 eyr:1967 hgt:170cm\n\
                       ecl:grn pid:012533040 byr:1946\n\
                       \n\
                       hcl:dab227 iyr:2012\n\
                       ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
                       \n\
                       hgt:59cm ecl:zzz\n\
                       eyr:2038 hcl:74454a iyr:2023\n\
                       pid:3556412378 byr:2007";

        let example2 = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
                       hcl:#623a2f\n\
                       \n\
                       eyr:2029 ecl:blu cid:129 byr:1989\n\
                       iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
                       \n\
                       hcl:#888785\n\
                       hgt:164cm byr:2001 iyr:2015 cid:88\n\
                       pid:545766238 ecl:hzl\n\
                       eyr:2022\n\
                       \n\
                       iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(solve_b(example1), Some(0));
        assert_eq!(solve_b(example2), Some(4));
    }
}
