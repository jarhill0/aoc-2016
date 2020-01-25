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

    fn part2(&self, input: String) {
        print!(
            "{}",
            parse_input_2(&input)
                .filter(satisfies_triangle_inequality)
                .count()
        );
    }
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

fn parse_input_2(input: &str) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
    ColumnIterator {
        line_iterator: input.split('\n'),
        triplet_a: None,
        triplet_b: None,
    }
}

struct ColumnIterator<'a, T: Iterator<Item = &'a str>> {
    line_iterator: T,

    triplet_a: Option<(i32, i32, i32)>,
    triplet_b: Option<(i32, i32, i32)>,
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for ColumnIterator<'a, T> {
    type Item = (i32, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        // stored an answer from a previous iteration…
        if let Some(triplet) = self.triplet_a {
            self.triplet_a = None;
            return Some(triplet);
        }
        if let Some(triplet) = self.triplet_b {
            self.triplet_b = None;
            return Some(triplet);
        }

        let mut lines = vec![];

        for _ in 0..3 {
            let new_line = self.line_iterator.next();
            lines.push(new_line?.split_whitespace());
        }

        let mut l1 = lines.pop().unwrap();
        let mut l2 = lines.pop().unwrap();
        let mut l3 = lines.pop().unwrap();

        self.triplet_a = Some((
            int(l1.next().unwrap()).unwrap(),
            int(l2.next().unwrap()).unwrap(),
            int(l3.next().unwrap()).unwrap(),
        ));
        self.triplet_b = Some((
            int(l1.next().unwrap()).unwrap(),
            int(l2.next().unwrap()).unwrap(),
            int(l3.next().unwrap()).unwrap(),
        ));
        Some((
            int(l1.next().unwrap()).unwrap(),
            int(l2.next().unwrap()).unwrap(),
            int(l3.next().unwrap()).unwrap(),
        ))
    }
}
