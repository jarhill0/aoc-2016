from os import path
from sys import argv, exit, stderr


def main():
    if len(argv) != 3:
        print(f"Usage: {argv[0]} <daynum> <dir>", file=stderr)
        return 1

    _, day, directory = argv
    try:
        day = int(day)
    except ValueError:
        print(f"Not a number: {day!r}", file=stderr)
        return 1

    with open(path.join(directory, f"day{day:02d}.rs"), "w") as f:
        f.write(TEMPLATE.format(day=day))
    return 0


TEMPLATE = """
use crate::solutions::Solution;

pub struct Day{day} {{
}}

impl Solution for Day{day} {{
    fn part1(&self, input: String) {{
    }}

    fn part2(&self, input: String) {{
    }}
}}
"""


if __name__ == "__main__":
    exit(main())
