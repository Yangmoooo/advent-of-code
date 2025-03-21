import argparse
import subprocess

default_year = 2020


def arg_parser():
    parser = argparse.ArgumentParser(description="AoC Helper")
    subparsers = parser.add_subparsers(
        dest="command", help="Available commands", required=True
    )
    year_msg = f"Year of the challenge, default is {default_year}"
    day_msg = "Day of the challenge"

    # fetch 子命令
    parser_fetch = subparsers.add_parser(
        "fetch",
        aliases=["f"],
        help="Download the input with aoc-cli",
    )
    parser_fetch.add_argument("year", nargs="?", type=int, help=year_msg)
    parser_fetch.add_argument("day", type=int, help=day_msg)

    # new 子命令
    parser_new = subparsers.add_parser(
        "new", aliases=["n"], help="Create a new solution"
    )
    parser_new.add_argument("year", nargs="?", type=int, help=year_msg)
    parser_new.add_argument("day", type=int, help=day_msg)
    parser_new.add_argument(
        "-f", "--fetch", action="store_true", help="Fetch before create"
    )

    # test 子命令
    parser_test = subparsers.add_parser("test", aliases=["t"], help="Test the solution")
    parser_test.add_argument("year", nargs="?", type=int, help=year_msg)
    parser_test.add_argument("day", type=int, help=day_msg)

    # run 子命令
    parser_run = subparsers.add_parser("run", aliases=["r"], help="Run the solution")
    parser_run.add_argument("year", nargs="?", type=int, help=year_msg)
    parser_run.add_argument("day", type=int, help=day_msg)
    parser_run.add_argument("-t", "--test", action="store_true", help="Test before run")

    return parser


def aoc_download(year, day, input_file):
    download_cmd = [
        "aoc",
        "download",
        "--year",
        year,
        "--day",
        day,
        "--input-only",
        "--input-file",
        input_file,
    ]
    exitcode = subprocess.run(download_cmd)
    if exitcode.returncode != 0:
        exit(1)


def aoc_fetch(year, day):
    input_file = f"aoc{year}/inputs/day{day}.txt"
    aoc_download(year, day, input_file)


def aoc_new(year, day, fetch_first):
    input_file = f"aoc{year}/inputs/day{day}.txt"
    if fetch_first:
        aoc_download(year, day, input_file)
    src_file = f"aoc{year}/src/bin/day{day}.rs"
    with open(src_file, "x") as f:
        f.write(f"use aoc{year}::*;\n\n")
        f.write("fn main() {\n")
        f.write(f'    let input = read_file("{input_file}");\n')
        f.write("    let answer_a = solve_a(&input);\n")
        f.write("    let answer_b = solve_b(&input);\n")
        f.write(f"    print_answer({int(day)}, (answer_a, answer_b));\n")
        f.write("}\n\n")
        f.write("fn solve_a(input: &str) -> Option<usize> {\n")
        f.write("    None\n")
        f.write("}\n\n")
        f.write("fn solve_b(input: &str) -> Option<usize> {\n")
        f.write("    None\n")
        f.write("}\n\n")


def aoc_test(year, day):
    test_cmd = [
        "cargo",
        "test",
        "--package",
        "aoc" + year,
        "--bin",
        "day" + day,
    ]
    subprocess.run(test_cmd)


def aoc_run(year, day, test_first):
    run_cmd = [
        "cargo",
        "run",
        "--package",
        "aoc" + year,
        "--bin",
        "day" + day,
    ]

    if test_first:
        test_cmd = [
            "cargo",
            "test",
            "--package",
            "aoc" + year,
            "--bin",
            "day" + day,
        ]
        exitcode = subprocess.run(test_cmd)
        if exitcode.returncode == 0:
            subprocess.run(run_cmd)
    else:
        subprocess.run(run_cmd)


def main():
    parser = arg_parser()
    args = parser.parse_args()

    if args.year is None:
        args.year = default_year

    year = str(args.year)
    day = "{:02d}".format(args.day)

    match args.command:
        case "fetch" | "f":
            aoc_fetch(year, day)
        case "new" | "n":
            aoc_new(year, day, args.fetch)
        case "test" | "t":
            aoc_test(year, day)
        case "run" | "r":
            aoc_run(year, day, args.test)
        case _:
            parser.print_help()


if __name__ == "__main__":
    main()
