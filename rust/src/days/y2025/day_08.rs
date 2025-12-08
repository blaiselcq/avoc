use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: u64,
    y: u64,
    z: u64,
}

fn load_input(input: &str) -> Vec<Position> {
    input
        .lines()
        .map(|line| line.split(','))
        .map(|mut p| Position {
            x: p.next().unwrap().parse().unwrap(),
            y: p.next().unwrap().parse().unwrap(),
            z: p.next().unwrap().parse().unwrap(),
        })
        .collect()
}

fn distance_sq(lhs: &Position, rhs: &Position) -> u64 {
    lhs.x.abs_diff(rhs.x).pow(2) + lhs.y.abs_diff(rhs.y).pow(2) + lhs.z.abs_diff(rhs.z).pow(2)
}

fn get_distances(positions: &[Position]) -> impl Iterator<Item = ((u16, u16), u64)> + use<'_> {
    positions.iter().enumerate().flat_map(|(iy, y)| {
        positions
            .iter()
            .enumerate()
            .filter_map(move |(ix, x)| match ix > iy {
                true => Some(((ix as u16, iy as u16), distance_sq(x, y))),
                false => None,
            })
    })
}

fn add_to_groups(mut groups: Vec<BTreeSet<u16>>, c: (u16, u16)) -> Vec<BTreeSet<u16>> {
    let g0 = groups
        .iter()
        .enumerate()
        .find(|(_i, r)| r.contains(&c.0))
        .map(|p| p.0);
    let g1 = groups
        .iter()
        .enumerate()
        .find(|(_i, r)| r.contains(&c.1))
        .map(|p| p.0);
    if g0.is_some() && g1.is_some() && g0.unwrap() != g1.unwrap() {
        let mut to_cpy = groups[g1.unwrap()].clone();
        groups[g0.unwrap()].append(&mut to_cpy);
        groups.remove(g1.unwrap());
    }
    let group = if let Some(group) = groups
        .iter_mut()
        .find(|r| r.contains(&c.1) || r.contains(&c.0))
    {
        group
    } else {
        groups.push(BTreeSet::new());
        groups.last_mut().unwrap()
    };
    group.insert(c.0);
    group.insert(c.1);

    groups
}

pub fn puzzle_1(input: &str) -> String {
    let input = load_input(input);
    let is_test = input.len() == 20;

    let mut distances = get_distances(&input).collect::<Vec<_>>();
    let pair_count = if is_test { 10 } else { 1000 };
    distances.sort_by_key(|p| p.1);
    let groups = distances
        .iter()
        .take(pair_count)
        .map(|p| p.0)
        .fold(vec![], add_to_groups);

    let mut group_counts = groups.iter().map(|s| s.len()).collect::<Vec<_>>();
    group_counts.sort();
    let res = group_counts
        .into_iter()
        .rev()
        .take(3)
        .reduce(|acc, x| acc * x)
        .unwrap();

    res.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input = load_input(input);

    let mut distances = get_distances(&input).collect::<Vec<_>>();
    distances.sort_by_key(|p| p.1);

    let mut groups = vec![];
    let mut breaking_connection = (0, 0);
    for (c, _d) in distances {
        groups = add_to_groups(groups, c);
        if groups[0].len() == input.len() {
            breaking_connection = c;
            break;
        }
    }

    let res = input[breaking_connection.0 as usize].x * input[breaking_connection.1 as usize].x;
    res.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "40");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "25272");
    }
}
