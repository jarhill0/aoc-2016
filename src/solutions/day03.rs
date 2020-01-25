use crate::solutions::Solution;
use crate::util::int;

pub struct Day3 {}

impl Solution for Day3 {
    fn part1(&self, input: String) {
        print!(
            "{}",
            parse_input(&input)
                .filter(satisfies_triangle_inequality)
                .count()
        );
    }

    fn part2(&self, input: String) {}
}

fn satisfies_triangle_inequality(triangle: &(i32, i32, i32)) -> bool {
    let (short, mid, long) = sort_tuple(*triangle);
    short + mid > long
}

fn sort_tuple(tuple: (i32, i32, i32)) -> (i32, i32, i32) {
    // from https://users.rust-lang.org/t/sort-3-tuple-idiomatic-code/15913/2
    let (a, b, c) = tuple;
    let mut v = [a, b, c];
    v.sort();
    let [a, b, c] = v;
    (a, b, c)
}

fn parse_input(input: &str) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
    input.split('\n').map(|line| {
        let mut words = line.split_whitespace();
        (
            int(words.next().unwrap()).unwrap(),
            int(words.next().unwrap()).unwrap(),
            int(words.next().unwrap()).unwrap(),
        )
    })
}
