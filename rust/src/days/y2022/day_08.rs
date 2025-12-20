use crate::utils::geometry::Point2;

type Point = Point2<usize>;
type TreeGrid = Vec<Vec<u8>>;

#[derive(Clone, Copy, Debug)]
enum Side {
    North,
    West,
    South,
    East,
}

//      N
//   |------> y
// W |         E
//   \/
//   x
//      S

fn load_input(input: &str) -> Result<TreeGrid, ()> {
    let output: Vec<Vec<u8>> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|d| d.try_into().unwrap()).unwrap())
                .collect()
        })
        .collect();

    if !output.iter().map(|l| l.len()).all(|x| x == output.len()) {
        return Err(());
    }

    Ok(output)
}

fn is_visible_from_side(forest: &TreeGrid, side: Side, pos: Point) -> bool {
    let directly_visible = match side {
        Side::North => pos.x == 0,
        Side::West => pos.y == 0,
        Side::South => pos.x == forest.len() - 1,
        Side::East => pos.y == forest.first().unwrap().len() - 1,
    };

    if directly_visible {
        return true;
    }

    let tree_height = &forest[pos.x][pos.y];

    let borns = match side {
        Side::North => 0..pos.x,
        Side::West => 0..pos.y,
        Side::South => pos.x + 1..forest.len(),
        Side::East => pos.y + 1..forest.get(pos.x).unwrap().len(),
    };

    match side {
        Side::North | Side::South => {
            &forest[borns]
                .iter()
                .map(|f| f.get(pos.y).unwrap())
                .max()
                .unwrap()
                .clone()
                < tree_height
        }
        Side::West | Side::East => {
            &forest.get(pos.x).unwrap()[borns]
                .iter()
                .max()
                .unwrap()
                .clone()
                < tree_height
        }
    }
}

fn get_visibility_length(forest: &TreeGrid, side: Side, pos: Point) -> usize {
    let on_edge = match side {
        Side::North => pos.x == 0,
        Side::West => pos.y == 0,
        Side::South => pos.x == forest.len() - 1,
        Side::East => pos.y == forest.first().unwrap().len() - 1,
    };

    if on_edge {
        return 0;
    }

    let tree_height = &forest[pos.x][pos.y];

    let borns = match side {
        Side::North => 0..pos.x,
        Side::West => 0..pos.y,
        Side::South => pos.x + 1..forest.len(),
        Side::East => pos.y + 1..forest.get(pos.x).unwrap().len(),
    };

    match side {
        Side::North => forest[borns.clone()]
            .iter()
            .rev()
            .map(|f| f.get(pos.y).unwrap())
            .collect::<Vec<_>>(),

        Side::South => forest[borns.clone()]
            .iter()
            .map(|f| f.get(pos.y).unwrap())
            .collect(),

        Side::West => forest.get(pos.x).unwrap()[borns.clone()]
            .iter()
            .rev()
            .collect(),

        Side::East => forest.get(pos.x).unwrap()[borns.clone()].iter().collect(),
    }
    .iter()
    .map(|&t| t < tree_height)
    .position(|r| !r)
    .map_or(borns.count(), |v| v + 1)
}

fn is_tree_visible(forest: &TreeGrid, pos: Point) -> bool {
    [Side::North, Side::West, Side::South, Side::East]
        .iter()
        .map(|side| is_visible_from_side(forest, *side, pos))
        .any(|x| x)
}

fn get_visible_trees(input: &TreeGrid) -> Vec<Vec<bool>> {
    let mut output = vec![vec![false; input.first().unwrap().len()]; input.len()];
    output.iter_mut().enumerate().for_each(|(i, output_row)| {
        output_row
            .iter_mut()
            .enumerate()
            .for_each(|(j, visible)| *visible = is_tree_visible(input, Point { x: i, y: j }));
    });

    output
}

fn get_visibility_scores(input: &TreeGrid) -> Vec<Vec<usize>> {
    let mut output = vec![vec![0; input.first().unwrap().len()]; input.len()];

    output.iter_mut().enumerate().for_each(|(i, output_row)| {
        output_row.iter_mut().enumerate().for_each(|(j, visible)| {
            *visible = get_visibility_length(input, Side::North, Point { x: i, y: j })
                * get_visibility_length(input, Side::West, Point { x: i, y: j })
                * get_visibility_length(input, Side::South, Point { x: i, y: j })
                * get_visibility_length(input, Side::East, Point { x: i, y: j })
        });
    });

    output
}

pub fn puzzle_1(input: &str) -> String {
    let parsed_input = load_input(input).unwrap();
    let visible_trees = get_visible_trees(&parsed_input);

    let total: usize = visible_trees
        .iter()
        .map(|trees| {
            trees
                .iter()
                .map(|&x| match x {
                    true => 1,
                    false => 0,
                })
                .sum::<usize>()
        })
        .sum();

    total.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let parsed_input = load_input(input).unwrap();
    let visibility_scores = get_visibility_scores(&parsed_input);

    visibility_scores
        .iter()
        .map(|v| v.iter().max().unwrap())
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "30373
25512
65332
33549
35390";

    use super::*;

    #[test]
    fn test_load_input() {
        let input = "12\n34\n";
        let ouput = load_input(input);
        assert_eq!(ouput, Ok(vec![vec![1, 2], vec![3, 4]]));
    }

    #[test]
    fn test_is_tree_visible() {
        let forest = load_input(INPUT).unwrap();

        assert!(is_tree_visible(&forest, Point { x: 0, y: 0 }));
        assert!(is_tree_visible(&forest, Point { x: 1, y: 1 }));
        assert!(!is_tree_visible(&forest, Point { x: 1, y: 3 }));
        assert!(!is_tree_visible(&forest, Point { x: 2, y: 2 }));
    }

    #[test]
    fn test_get_visible_trees() {
        let input = "123\n416\n123\n";
        let forest = load_input(input).unwrap();

        let visible_trees = get_visible_trees(&forest);
        assert_eq!(
            visible_trees,
            vec![
                vec![true, true, true],
                vec![true, false, true],
                vec![true, true, true]
            ]
        );
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "21");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "8");
    }
}
