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
        help="Download the input using aoc-cli. Prepare the src and Cargo.toml",
    )
    parser_fetch.add_argument("year", nargs="?", type=int, help=year_msg)
    parser_fetch.add_argument("day", type=int, help=day_msg)

    # run 子命令
    parser_run = subparsers.add_parser("run", aliases=["r"], help="Run the solution")
    parser_run.add_argument("year", nargs="?", type=int, help=year_msg)
    parser_run.add_argument("day", type=int, help=day_msg)
    parser_run.add_argument("-t", "--test", action="store_true", help="Test before run")

    # test 子命令
    parser_test = subparsers.add_parser("test", aliases=["t"], help="Test the solution")
    parser_test.add_argument("year", nargs="?", type=int, help=year_msg)
    parser_test.add_argument("day", type=int, help=day_msg)

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
    subprocess.run(download_cmd)


def create_src_file(src_file, year, day, input_file):
    with open(src_file, "x") as f:
        f.write(f"use aoc{year}::{{print_answer, read_file}};\n\n")
        f.write("fn main() {\n")
        f.write(f'    let input = read_file("{input_file}");\n')
        f.write(f"    print_answer({day}, false, solve1(&input));\n")
        f.write("}\n")


def add_cargo_bin(cargo_toml, day):
    with open(cargo_toml, "a") as f:
        f.write(f'\n[[bin]]\nname = "day{day}"\npath = "src/day{day}.rs"\n')


def aoc_fetch(year, day):
    raw_day = day
    day = "{:02d}".format(day)
    input_file = f"aoc{year}/inputs/day{day}.txt"
    aoc_download(year, day, input_file)
    src_file = f"aoc{year}/src/day{day}.rs"
    cargo_toml = f"aoc{year}/Cargo.toml"
    create_src_file(src_file, year, raw_day, input_file)
    add_cargo_bin(cargo_toml, day)


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


def main():
    parser = arg_parser()
    args = parser.parse_args()

    if args.year is None:
        args.year = default_year

    year = str(args.year)
    day = "{:02d}".format(args.day)

    match args.command:
        case "fetch" | "f":
            aoc_fetch(year, args.day)
        case "run" | "r":
            aoc_run(year, day, args.test)
        case "test" | "t":
            aoc_test(year, day)
        case _:
            parser.print_help()


if __name__ == "__main__":
    main()
