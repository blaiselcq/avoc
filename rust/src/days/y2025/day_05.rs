use std::{collections::BTreeSet, ops::RangeInclusive};

fn load_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    (
        ranges
            .lines()
            .map(|l| {
                let (min, max) = l.split_once('-').unwrap();
                min.parse().unwrap()..=max.parse().unwrap()
            })
            .collect(),
        ingredients
            .trim()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect(),
    )
}

pub fn puzzle_1(input: &str) -> String {
    let input = load_input(input);

    let res = input
        .1
        .iter()
        .filter(|&id| input.0.iter().any(|r| r.contains(id)))
        .count();

    res.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input = load_input(input);

    let mut ranges = input.0;

    loop {
        let extended_ranges = ranges
            .iter()
            .enumerate()
            .map(|(id, range)| {
                let min = ranges
                    .iter()
                    .skip(id)
                    .filter_map(|r| match r.contains(range.start()) {
                        true => Some(r.start()),
                        false => None,
                    })
                    .min();
                let max = ranges
                    .iter()
                    .skip(id)
                    .filter_map(|r| match r.contains(range.end()) {
                        true => Some(r.end()),
                        false => None,
                    })
                    .max();

                *min.unwrap_or(range.start())..=*max.unwrap_or(range.end())
            })
            .collect::<Vec<_>>();

        if extended_ranges == ranges {
            break;
        }
        ranges = extended_ranges;
    }

    let res = ranges
        .iter()
        .filter(|range| {
            !ranges.iter().any(|other| {
                other != *range && (other.contains(range.start()) && other.contains(range.end()))
            })
        })
        .map(|r| (*r.start(), *r.end()))
        .collect::<BTreeSet<_>>() // dedup
        .iter()
        .map(|(s, e)| e - s + 1)
        .sum::<usize>();

    res.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "3");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "14");
    }
}
