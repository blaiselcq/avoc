use std::collections::BTreeSet;

use crate::utils::geometry::{point2, Point2};

type Point = Point2<i16>;

fn load_input(input: &str) -> BTreeSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| match c {
                '@' => Some(Point {
                    x: j as i16,
                    y: i as i16,
                }),
                _ => None,
            })
        })
        .collect()
}

fn has_enough_neighbours(point: Point, map: &BTreeSet<Point>, limit: usize) -> bool {
    [
        point2!(-1, 0),
        point2!(1, 0),
        point2!(0, -1),
        point2!(0, 1),
        point2!(-1, -1),
        point2!(-1, 1),
        point2!(1, -1),
        point2!(1, 1),
    ]
    .iter()
    .filter(|&&neighbour| map.contains(&(point - neighbour)))
    .count()
        >= limit
}

pub fn puzzle_1(input: &str) -> String {
    let input = load_input(input);
    let res = input
        .iter()
        .filter(|&&point| !has_enough_neighbours(point, &input, 4))
        .count();
    res.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut input = load_input(input);
    let mut count = 0;
    loop {
        let accessible = input
            .clone()
            .into_iter()
            .filter(|&point| !has_enough_neighbours(point, &input, 4))
            .collect::<Vec<_>>();

        if accessible.is_empty() {
            break;
        }
        count += accessible.len();

        for point in accessible {
            input.remove(&point);
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "13");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "43");
    }
}
