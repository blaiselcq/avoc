use std::collections::BTreeMap;

use crate::utils::geometry::{Line2, Point2};

type Point = Point2<i16>;
type Line = Line2<i16>;

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (a_str, b_str) = line.split_once(" -> ").unwrap();
            Line {
                start: {
                    let (x, y) = a_str.split_once(',').unwrap();
                    let x = x.parse().unwrap();
                    let y = y.parse().unwrap();
                    Point { x, y }
                },
                end: {
                    let (x, y) = b_str.split_once(',').unwrap();
                    let x = x.parse().unwrap();
                    let y = y.parse().unwrap();
                    Point { x, y }
                },
            }
        })
        .collect()
}

fn is_aligned_with_grid(line: &Line) -> bool {
    line.start.x == line.end.x || line.start.y == line.end.y
}

fn intersect_map(input: &Vec<Line>) -> BTreeMap<Point, usize> {
    let mut res = BTreeMap::new();
    for line in input {
        for p in line.into_iter() {
            res.entry(p).and_modify(|c| *c += 1).or_insert(1);
        }
    }

    res
}

pub fn puzzle_1(input: &str) -> String {
    let input = parse_input(input)
        .into_iter()
        .filter(is_aligned_with_grid)
        .collect();
    let intersections = intersect_map(&input);

    intersections
        .iter()
        .filter(|(_, &c)| c > 1)
        .count()
        .to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input = parse_input(input);
    let intersections = intersect_map(&input);

    intersections
        .iter()
        .filter(|(_, &c)| c > 1)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    use super::*;

    #[test]
    fn test_can_parse_input() {
        let parsed = parse_input(INPUT);
        assert_eq!(
            parsed.first(),
            Some(&Line {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 }
            })
        );
        assert_eq!(parsed.len(), 10);
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "5");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "12");
    }
}
