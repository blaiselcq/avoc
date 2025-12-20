use std::collections::BTreeSet;

use crate::utils::geometry::Point2;

type Point = Point2<i32>;
type Vector = Point2<i32>;

struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn new(knots: usize, starting_pos: Point) -> Rope {
        Rope {
            knots: vec![starting_pos; knots],
        }
    }

    fn get_tail(&self) -> Option<&Point> {
        return self.knots.last();
    }

    fn move_head(&mut self, vector: Vector) -> Vec<Point> {
        let mut tail_moves = vec![];

        let distance = vector.norm_1();
        for _ in 0..distance {
            if let Some(head) = self.knots.get_mut(0) {
                *head += vector / distance;
            } else {
                return tail_moves;
            }

            for id in 1..self.knots.len() {
                let last_knot = self.knots[id - 1];

                let dx = last_knot.x - self.knots[id].x;
                let dy = last_knot.y - self.knots[id].y;

                if dx.abs().max(dy.abs()) <= 1 {
                    continue;
                }

                self.knots[id] += Vector::unit_x() * dx.signum();
                self.knots[id] += Vector::unit_y() * dy.signum();
                if id == self.knots.len() - 1 {
                    tail_moves.push(self.knots[id]);
                }
            }
        }

        tail_moves
    }
}

#[allow(dead_code)]
fn print_debug_pos(input: &BTreeSet<Point>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;

    input.iter().for_each(|Point { x, y }| {
        if x < &min_x {
            min_x = *x;
        }
        if y < &min_y {
            min_y = *y;
        }
        if x > &max_x {
            max_x = *x;
        }
        if y > &max_y {
            max_y = *y;
        }
    });

    for y in (min_y..=max_y).rev() {
        let mut line = String::new();
        line.push_str(&format!("{}\t", y));
        for x in min_x..=max_x {
            if x == 0 && y == 0 {
                line.push('s');
            } else if input.contains(&Point { x, y }) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{line}");
    }
}

fn parse_direction(value: char) -> Result<Point, ()> {
    match value {
        'U' => Ok(Point::unit_y()),
        'D' => Ok(-Point::unit_y()),
        'R' => Ok(Point::unit_x()),
        'L' => Ok(-Point::unit_x()),
        _ => Err(()),
    }
}

fn parse_input(input: &str) -> Vec<Vector> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (direction, distance) = l.split_once(' ').unwrap();
            let distance = distance.parse().ok().unwrap();
            let vector = direction
                .chars()
                .next()
                .and_then(|c| parse_direction(c).ok())
                .unwrap()
                * distance;
            vector
        })
        .collect()
}

fn get_tail_pos(mouvements: &Vec<Vector>, rope: &mut Rope) -> BTreeSet<Point> {
    let mut positions = BTreeSet::new();
    positions.insert(*rope.get_tail().unwrap());

    for vector in mouvements {
        for pos_tail in rope.move_head(*vector) {
            positions.insert(pos_tail);
        }
        // print_debug_pos(&positions);
    }

    positions
}

pub fn puzzle_1(input: &str) -> String {
    let mouvements = parse_input(input);

    let mut rope = Rope::new(2, Point { x: 0, y: 0 });

    let positions = get_tail_pos(&mouvements, &mut rope);
    positions.len().to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mouvements = parse_input(input);

    let mut rope = Rope::new(10, Point { x: 0, y: 0 });

    let positions = get_tail_pos(&mouvements, &mut rope);
    positions.len().to_string()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    use super::*;

    #[test]
    fn test_move_head_and_tail() {
        let mut rope = Rope {
            knots: vec![Point { x: 3, y: 3 }, Point { x: 2, y: 2 }],
        };
        let tail_pos = rope.move_head(Vector::unit_y());
        assert_eq!(tail_pos.len(), 1);
        assert_eq!(rope.get_tail().unwrap().clone(), Point { x: 3, y: 3 });
    }

    #[test]
    fn test_parse_input() {
        let input = "R 4\nU 4\n";
        assert_eq!(
            parse_input(input),
            vec![Vector::unit_x() * 4, Vector::unit_y() * 4]
        );
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "88");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "36");
    }
}
