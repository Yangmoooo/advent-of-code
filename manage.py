import argparse
import subprocess

default_year = 2020


def main():
    parser = argparse.ArgumentParser(description="AoC Helper")
    subparsers = parser.add_subparsers(
        dest="command", help="Available commands", required=True
    )
    year_msg = f"Year of the challenge, default is {default_year}"
    day_msg = "Day of the challenge"

    # download 子命令
    parser_download = subparsers.add_parser(
        "download", aliases=["d"], help="Download the puzzle and input, need aoc-cli"
    )
    parser_download.add_argument("year", nargs="?", type=int, help=year_msg)
    parser_download.add_argument("day", type=int, help=day_msg)

    # run 子命令
    parser_run = subparsers.add_parser("run", aliases=["r"], help="Run the solution")
    parser_run.add_argument("year", nargs="?", type=int, help=year_msg)
    parser_run.add_argument("day", type=int, help=day_msg)
    parser_run.add_argument("-t", "--test", action="store_true", help="Test before run")

    # test 子命令
    parser_test = subparsers.add_parser("test", aliases=["t"], help="Test the solution")
    parser_test.add_argument("year", nargs="?", type=int, help=year_msg)
    parser_test.add_argument("day", type=int, help=day_msg)

    args = parser.parse_args()

    if args.year is None:
        args.year = default_year

    day = "{:02d}".format(args.day)
    name = "day" + day
    year = str(args.year)
    pkg = "aoc" + year
    subcommand = args.command

    if subcommand == "download" or subcommand == "d":
        puzzle_file = f"{pkg}/puzzles/{name}.md"
        input_file = f"{pkg}/inputs/{name}.txt"
        download_cmd = [
            "aoc",
            "download",
            "--year",
            year,
            "--day",
            day,
            "--puzzle-file",
            puzzle_file,
            "--input-file",
            input_file,
        ]
        subprocess.run(download_cmd)
    else:
        run_cmd = [
            "cargo",
            "run",
            "--package",
            pkg,
            "--bin",
            name,
        ]
        test_cmd = [
            "cargo",
            "test",
            "--package",
            pkg,
            "--bin",
            name,
        ]
        match args.command:
            case "run" | "r":
                if args.test:
                    exitcode = subprocess.run(test_cmd)
                    if exitcode.returncode == 0:
                        subprocess.run(run_cmd)
                else:
                    subprocess.run(run_cmd)
            case "test" | "t":
                subprocess.run(test_cmd)
            case _:
                parser.print_help()


if __name__ == "__main__":
    main()
