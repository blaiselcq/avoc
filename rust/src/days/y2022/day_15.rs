use std::{collections::BTreeMap, ops::RangeInclusive};

use crate::utils::geometry::Point2;

type Point = Point2<i32>;

pub(crate) struct Sensor {
    closest_beacon: Point,
    distance: i32,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum MapState {
    Unknown,
    Empty,
    Beacon,
    Sensor,
}

mod parser {
    use std::collections::BTreeMap;

    use nom::{
        bytes::complete::tag,
        character::complete::{self, line_ending},
        multi::separated_list1,
        sequence::{preceded, separated_pair},
        IResult,
    };

    use super::{Point, Sensor};

    fn parse_coord(input: &str) -> IResult<&str, Point> {
        let (input, coord) = separated_pair(
            preceded(tag("x="), complete::i32),
            tag(", "),
            preceded(tag("y="), complete::i32),
        )(input)?;
        let coord = Point {
            x: coord.0,
            y: coord.1,
        };

        Ok((input, coord))
    }

    fn parse_line(input: &str) -> IResult<&str, (Point, Sensor)> {
        let (input, (sensor, closest_beacon)) = separated_pair(
            preceded(tag("Sensor at "), parse_coord),
            tag(": "),
            preceded(tag("closest beacon is at "), parse_coord),
        )(input)?;

        let distance = sensor.distance_1(&closest_beacon);
        Ok((
            input,
            (
                sensor,
                Sensor {
                    closest_beacon,
                    distance,
                },
            ),
        ))
    }

    pub(crate) fn parse_input(input: &str) -> BTreeMap<Point, Sensor> {
        let (_, result) = separated_list1(line_ending, parse_line)(input).unwrap();

        result.into_iter().collect()
    }
}

fn get_footprint(map: &BTreeMap<Point, Sensor>) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    if map.is_empty() {
        return ((0..=0), (0..=0));
    }

    let x_min = map
        .iter()
        .map(|(sensor, closest_beacon)| sensor.x - closest_beacon.distance)
        .min()
        .unwrap();
    let x_max = map
        .iter()
        .map(|(sensor, closest_beacon)| sensor.x + closest_beacon.distance)
        .max()
        .unwrap();
    let y_min = map
        .iter()
        .map(|(sensor, closest_beacon)| sensor.y - closest_beacon.distance)
        .min()
        .unwrap();
    let y_max = map
        .iter()
        .map(|(sensor, closest_beacon)| sensor.y + closest_beacon.distance)
        .max()
        .unwrap();

    ((x_min..=x_max), (y_min..=y_max))
}

fn get_row(
    row_number: i32,
    map: &BTreeMap<Point, Sensor>,
    footprint: (RangeInclusive<i32>, RangeInclusive<i32>),
) -> Vec<MapState> {
    let (x_range, _) = footprint;
    let min_x = x_range.clone().min().unwrap();

    let mut result = vec![MapState::Unknown; x_range.count()];

    let filtered_data = map.iter().filter(|(pos_sensor, sensor)| {
        let distance: i32 = pos_sensor.y.abs_diff(row_number).try_into().unwrap();
        distance <= sensor.distance
    });

    for (position, sensor) in filtered_data {
        let distance: i32 = position.y.abs_diff(row_number).try_into().unwrap();
        let start: i32 = position.x - (sensor.distance - distance);
        let end: i32 = position.x + (sensor.distance - distance);
        for i in start..=end {
            result[(i - min_x) as usize] = MapState::Empty;
        }
        if position.y == row_number {
            result[(position.x - min_x) as usize] = MapState::Sensor;
        }
        if sensor.closest_beacon.y == row_number {
            result[(sensor.closest_beacon.x - min_x) as usize] = MapState::Beacon;
        }
    }

    result
}

fn get_first_empty_position(subgrid_size: i32, map: &BTreeMap<Point, Sensor>) -> Option<Point> {
    for row_number in 0..=subgrid_size {
        let mut ranges = map
            .iter()
            .map(|(position, sensor)| {
                let distance: i32 = position.y.abs_diff(row_number).try_into().unwrap();
                let start: i32 = position.x - (sensor.distance - distance);
                let end: i32 = position.x + (sensor.distance - distance);

                (start.max(0), end)
            })
            .collect::<Vec<_>>();

        ranges.sort_by_key(|x| x.0);

        let mut covered_until = -1;
        for range in ranges {
            let min = range.0;
            let max = range.1;
            if max > subgrid_size {
                break;
            }

            if min > covered_until + 1 {
                return Some(Point {
                    x: covered_until + 1,
                    y: row_number,
                });
            }

            covered_until = covered_until.max(max);
        }
    }
    None
}

pub fn puzzle_1(input: &str) -> String {
    let row_number = 2_000_000;

    let map = parser::parse_input(input);
    let footprint = get_footprint(&map);
    get_row(row_number, &map, footprint)
        .iter()
        .filter(|&&state| state == MapState::Empty)
        .count()
        .to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let max_coord = 4_000_000;

    let map = parser::parse_input(input);
    let first_empty_position = get_first_empty_position(max_coord, &map).unwrap();

    let value = first_empty_position.x as i64 * max_coord as i64 + first_empty_position.y as i64;
    value.to_string()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        let row_number = 10;

        let map = parser::parse_input(INPUT);
        let footprint = get_footprint(&map);
        let result = get_row(row_number, &map, footprint)
            .iter()
            .filter(|&&state| state == MapState::Empty)
            .count();

        assert_eq!(result, 26);
    }

    #[test]
    fn test_puzzle_2() {
        let max_coord = 20;

        let map = parser::parse_input(INPUT);
        let first_empty_position = get_first_empty_position(max_coord, &map).unwrap();

        let value = first_empty_position.x as i64 * 4_000_000i64 + first_empty_position.y as i64;

        assert_eq!(value, 56000011);
    }
}
