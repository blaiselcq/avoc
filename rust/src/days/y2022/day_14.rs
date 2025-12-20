use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
    sequence::separated_pair,
};

use crate::utils::geometry::Point2;

type Point = Point2<usize>;

#[derive(Debug)]
struct Map {
    bottom_height: usize,
    offset_x: usize,
    data: Vec<Vec<bool>>,
}

impl Map {
    fn set_occupied(&mut self, point: Point) {
        let data_x = point.x - self.offset_x;
        self.data[point.y][data_x] = true;
    }
    fn is_occupied(&self, point: Point) -> bool {
        let data_x = point.x - self.offset_x;
        self.data[point.y][data_x]
    }
}

fn get_range(start: usize, finish: usize) -> RangeInclusive<usize> {
    match start < finish {
        true => start..=finish,
        false => finish..=start,
    }
}

fn parse_input(input: &str) -> Map {
    let coord_parser = map(
        separated_pair(
            nom::character::complete::u16::<&str, ()>,
            tag(","),
            nom::character::complete::u16::<&str, ()>,
        ),
        |(x, y)| (x as usize, y as usize),
    );

    let mut parser = separated_list1(newline, separated_list1(tag(" -> "), coord_parser));

    let (_, result) = parser(input).unwrap();

    let max_y = result.iter().flatten().max_by_key(|c| c.1).unwrap().1 + 1;

    let bottom_height = max_y + 1;
    let offset_x = 500 - bottom_height;

    let mut data: Vec<Vec<bool>> = vec![];
    data.reserve(bottom_height + 1);

    for y in 0..=bottom_height {
        data.insert(y, vec![false; 2 * bottom_height + 1]);
    }

    let mut output_map = Map {
        bottom_height,
        offset_x,
        data,
    };

    for path in result {
        path.windows(2).for_each(|segment| {
            let start = segment[0];
            let finish = segment[1];
            if start.0 != finish.0 {
                for x in get_range(start.0, finish.0) {
                    output_map.set_occupied(Point { x, y: start.1 });
                }
                return;
            }
            if start.1 != finish.1 {
                for y in get_range(start.1, finish.1) {
                    output_map.set_occupied(Point { x: start.0, y });
                }
            }
        });
    }

    output_map
}

fn sand_fall(map: &Map, point: Point) -> Point {
    if !map.is_occupied(point + Point::unit_y()) {
        return point + Point::unit_y();
    }
    if !map.is_occupied(point + Point::unit_y() - Point::unit_x()) {
        return point + Point::unit_y() - Point::unit_x();
    }
    if !map.is_occupied(point + Point::unit_y() + Point::unit_x()) {
        return point + Point::unit_y() + Point::unit_x();
    }
    point
}

fn pour_sand(map: &mut Map, has_floor: bool) -> bool {
    let mut coord = Point { x: 500, y: 0 };

    loop {
        let fallen_coord = sand_fall(map, coord);
        if coord == fallen_coord {
            map.set_occupied(coord);
            return has_floor && coord.y < 1;
        }
        if has_floor && fallen_coord.y == map.bottom_height {
            map.set_occupied(coord);
            return false;
        }
        if !has_floor && fallen_coord.y >= map.bottom_height {
            return true;
        }
        coord = fallen_coord;
    }
}

pub fn puzzle_1(input: &str) -> String {
    let mut count = 0;
    let mut map = parse_input(input);

    while !pour_sand(&mut map, false) {
        count += 1;
    }
    count.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mut count = 0;
    let mut map = parse_input(input);

    while !pour_sand(&mut map, true) {
        count += 1;
    }
    (count + 1).to_string()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    use super::*;

    #[test]
    fn test_can_parse_input() {
        let parsed = parse_input(INPUT);
        assert_eq!(parsed.data.iter().flatten().filter(|x| **x).count(), 20);
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "24");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "93");
    }
}
