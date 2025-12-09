use num::Integer;
use rayon::prelude::*;

use crate::structs::geometry::{point2, Line2, Point2};

type Point = Point2<i64>;
type Line = Line2<i64>;

fn load_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|l| {
            l.split_once(',')
                .map(|(x, y)| point2!(x.parse().unwrap(), y.parse().unwrap()))
        })
        .collect()
}

fn area(lhs: &Point, rhs: &Point) -> u64 {
    (lhs.x.abs_diff(rhs.x) + 1) * (lhs.y.abs_diff(rhs.y) + 1)
}

// https://en.wikipedia.org/wiki/Point_in_polygon#Winding_number_algorithm
// multiline must have closing point
fn is_inside(point: &Point, multiline: &[Point]) -> bool {
    let mut wn = 0;
    for line in multiline.windows(2) {
        let (a, b) = (&line[0], &line[1]);
        let is_horizontal = a.y == b.y;
        if a.x < point.x && !is_horizontal {
            continue;
        }

        let same_y_range = a.y.min(b.y) <= point.y && a.y.max(b.y) >= point.y;
        let same_x_range = a.x.min(b.x) <= point.x && a.x.max(b.x) >= point.x;

        if same_y_range && same_x_range {
            return true;
        }

        if same_y_range && !is_horizontal {
            wn += 1;
        }
    }

    wn.is_odd()
}

fn is_totally_inside(point_a: &Point, point_b: &Point, multiline: &[Point]) -> bool {
    let perimeter = [
        Line {
            start: point2!(point_a.x, point_a.y),
            end: point2!(point_a.x, point_b.y),
        },
        Line {
            start: point2!(point_a.x, point_b.y),
            end: point2!(point_b.x, point_b.y),
        },
        Line {
            start: point2!(point_b.x, point_b.y),
            end: point2!(point_b.x, point_a.y),
        },
        Line {
            start: point2!(point_b.x, point_a.y),
            end: point2!(point_a.x, point_a.y),
        },
    ];

    perimeter
        .iter()
        .all(|l| l.into_iter().all(|p| is_inside(&p, multiline)))
}

pub fn puzzle_1(input: &str) -> String {
    let input = load_input(input);

    let res = input
        .iter()
        .flat_map(|a| input.iter().map(|b| area(a, b)))
        .max()
        .unwrap();

    res.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input = load_input(input);

    let perimeter = {
        let mut res = input.clone();
        res.push(input.first().unwrap().clone());
        res
    };

    let mut i = 0;
    let size = input.len() * input.len();
    let res = input
        .iter()
        .flat_map(|a| input.iter().map(|b| ((*a, *b), area(a, b))))
        .inspect(|_| {
            println!("{}/{}", i, size);
            i += 1;
        })
        .par_bridge()
        .filter_map(
            |((a, b), area)| match is_totally_inside(&a, &b, &perimeter) {
                true => Some(area),
                false => None,
            },
        )
        .max()
        .unwrap();

    res.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    use super::*;

    #[test]
    fn test_is_inside() {
        let input = load_input(INPUT);

        let perimeter = {
            let mut res = input.clone();
            res.push(input.first().unwrap().clone());
            res
        };

        assert!(is_inside(&point2!(11, 1), &perimeter));
        assert!(is_inside(&point2!(10, 2), &perimeter));
        assert!(!is_inside(&point2!(9, 0), &perimeter));
        assert!(is_inside(&point2!(4, 5), &perimeter));
        assert!(!is_inside(&point2!(4, 2), &perimeter));
        assert!(!is_inside(&point2!(3, 7), &perimeter));
        assert!(!is_inside(&point2!(12, 8), &perimeter));
        assert!(is_inside(&point2!(2, 3), &perimeter));
        assert!(!is_inside(&point2!(1, 2), &perimeter));
        assert!(is_inside(&point2!(6, 3), &perimeter));
        assert!(is_inside(&point2!(8, 5), &perimeter));
        for point in input {
            assert!(is_inside(&point, &perimeter));
        }

        assert!(is_totally_inside(
            &point2!(9, 5),
            &point2!(2, 3),
            &perimeter
        ));
        assert!(is_totally_inside(
            &point2!(2, 3),
            &point2!(9, 5),
            &perimeter
        ));
        assert!(is_totally_inside(
            &point2!(9, 7),
            &point2!(9, 5),
            &perimeter
        ));
        assert!(!is_totally_inside(
            &point2!(11, 1),
            &point2!(2, 5),
            &perimeter
        ));
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "50");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "24");
    }
}
