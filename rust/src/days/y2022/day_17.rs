use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

use crate::utils::geometry::{point2, Point2, Vector2};

type Point = Point2<i32>;
type Vector = Vector2<i32>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone)]
struct Rock {
    parts: Vec<Point>,
    offset: Vector,
}

impl Rock {
    fn make(parts: Vec<Point>) -> Self {
        let parts = parts.into_iter();

        let offset_w = parts.clone().max_by_key(|p| p.x).unwrap().x;
        let offset_h = parts.clone().max_by_key(|p| p.y).unwrap().y;
        let offset = point2!(offset_w, offset_h);

        let parts = parts.collect();
        Self { parts, offset }
    }
}

fn rock_intersects(rock_1: &Rock, position_1: Point, rock_2: &Rock, position_2: Point) -> bool {
    rock_1
        .parts
        .iter()
        .cartesian_product(rock_2.parts.iter())
        .any(|(&lhs, &rhs)| (lhs + position_1) == (rhs + position_2))
}

fn get_rock(number: u64) -> Rock {
    match number % 5 {
        1 => Rock::make(vec![
            point2!(0, 0),
            point2!(1, 0),
            point2!(2, 0),
            point2!(3, 0),
        ]),
        2 => Rock::make(vec![
            point2!(1, 2),
            point2!(0, 1),
            point2!(1, 1),
            point2!(2, 1),
            point2!(1, 0),
        ]),
        3 => Rock::make(vec![
            point2!(2, 2),
            point2!(2, 1),
            point2!(0, 0),
            point2!(1, 0),
            point2!(2, 0),
        ]),
        4 => Rock::make(vec![
            point2!(0, 3),
            point2!(0, 2),
            point2!(0, 1),
            point2!(0, 0),
        ]),
        0 => Rock::make(vec![
            point2!(0, 0),
            point2!(0, 1),
            point2!(1, 0),
            point2!(1, 1),
        ]),
        _ => unreachable!(),
    }
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|c| match c {
            '>' => Some(Direction::Right),
            '<' => Some(Direction::Left),
            _ => None,
        })
        .collect()
}

// Returns the new position, if the rock is able to move
fn rock_moves(
    direction: Direction,
    fallen_rocks: &BTreeMap<i32, Vec<(Point, Rock)>>,
    rock: &Rock,
    position: Point,
) -> Option<Point> {
    let new_position = match direction {
        Direction::Left => position - Point::unit_x(),
        Direction::Right => position + Point::unit_x(),
        Direction::Down => position - Point::unit_y(),
    };

    if new_position.x < 0 || new_position.x + rock.offset.x >= 7 || new_position.y < 0 {
        return None;
    }

    let &higher_rock_y = fallen_rocks.last_key_value().unwrap_or((&0, &vec![])).0;

    if new_position.y > higher_rock_y {
        return Some(new_position);
    }
    let mut rocks_to_check = fallen_rocks
        .range(new_position.y..=higher_rock_y)
        .flat_map(|(_, v)| v);

    if rocks_to_check.any(|(other_position, other_rock)| {
        rock_intersects(rock, new_position, other_rock, *other_position)
    }) {
        return None;
    }

    Some(new_position)
}

fn get_floor(fallen_rocks: &BTreeMap<i32, Vec<(Point, Rock)>>) -> u8 {
    let &max_height = fallen_rocks.keys().max().unwrap();

    fallen_rocks
        .range(max_height - 6..=max_height)
        .flat_map(|(_, v)| v)
        .flat_map(|(coord, rocks)| rocks.parts.iter().map(|r| *coord + *r))
        .fold(0u8, |acc, coord| acc | (1u8 << (coord.x as u8)))
}

fn fall(
    input: &Vec<Direction>,
    rock_number: u64,
) -> (BTreeMap<i32, Vec<(Point, Rock)>>, Option<(u64, i32, u64)>) {
    let mut highest_point = 0; // floor height

    let input_size = input.len();

    // Rocks indexed by their highest point
    let mut fallen_rocks: BTreeMap<i32, Vec<(Point, Rock)>> = BTreeMap::new();

    let mut cycle_history: BTreeMap<u8, BTreeSet<(usize, usize)>> = BTreeMap::new();
    let mut floor_history: BTreeMap<u8, (u64, i32)> = BTreeMap::new();

    let mut cycle = None;

    let mut i: usize = 0;
    for rock_number in 1..=rock_number {
        let rock = get_rock(rock_number);
        let mut rock_position = Point {
            x: 2,
            y: highest_point + 3,
        };

        loop {
            let direction = input[i % input_size];
            i += 1;
            if let Some(new_position) = rock_moves(direction, &fallen_rocks, &rock, rock_position) {
                rock_position = new_position;
            }
            if let Some(new_position) =
                rock_moves(Direction::Down, &fallen_rocks, &rock, rock_position)
            {
                rock_position = new_position;
            } else {
                let rock_top = rock_position.y + rock.offset.y + 1;
                highest_point = highest_point.max(rock_top);
                fallen_rocks
                    .entry(rock_position.y + rock.offset.y)
                    .and_modify(|v| v.push((rock_position, rock.clone())))
                    .or_insert(vec![(rock_position, rock.clone())]);
                break;
            }
        }
        let floor = get_floor(&fallen_rocks);
        let cycle_key = (i % input_size, (rock_number % 5) as usize);
        let cycle_detected = match cycle_history.entry(floor) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(BTreeSet::from([cycle_key]));
                None
            }
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                match entry.get().contains(&cycle_key) {
                    true => {
                        let &last_floor_rock_number = floor_history.get(&floor).unwrap();
                        Some((last_floor_rock_number, rock_number, cycle_key))
                    }
                    false => {
                        entry.get_mut().insert(cycle_key);
                        None
                    }
                }
            }
        };
        floor_history.insert(floor, (rock_number, highest_point));
        if cycle.is_none() {
            cycle = cycle_detected.and_then(|(last, current, _key)| match floor == 127 {
                true => Some((current - last.0, highest_point - last.1, current)),
                false => None,
            });
        }
    }

    (fallen_rocks, cycle)
}

pub fn puzzle_1(input: &str) -> String {
    let input = parse_input(input);

    (fall(&input, 2022).0.keys().max().unwrap() + 1).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input = parse_input(input);

    let max_len = 1_000_000_000_000;

    let mut initial_guess = 10;
    let (length, height, start) = loop {
        match fall(&input, initial_guess).1 {
            Some((length, height, start)) => break (length, height, start),
            None => (),
        }
        initial_guess *= 2;
    };
    dbg!(length, height, start);
    let remaining_cycles = (max_len - start) / length;
    let result = remaining_cycles * height as u64;
    result.to_string()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "<><\n";
        let parsed = parse_input(input);
        assert_eq!(
            parsed,
            vec![Direction::Left, Direction::Right, Direction::Left]
        );
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "3068");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "1514285714288");
    }
}
