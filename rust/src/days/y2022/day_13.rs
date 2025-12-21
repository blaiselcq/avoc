use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListElement {
    Number(u8),
    List(Vec<ListElement>),
}

impl Ord for ListElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for ListElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ListElement::Number(a), ListElement::Number(b)) => a.partial_cmp(b),
            (ListElement::List(list_a), ListElement::List(list_b)) => list_a.partial_cmp(list_b),
            (ListElement::Number(a), ListElement::List(list_b)) => {
                vec![ListElement::Number(*a)].partial_cmp(list_b)
            }
            (ListElement::List(list_a), ListElement::Number(b)) => {
                list_a.partial_cmp(&vec![ListElement::Number(*b)])
            }
        }
    }
}

fn parse_input_list(input: &str) -> Option<ListElement> {
    if input.is_empty() {
        return None;
    }

    if !(input.starts_with('[') && input.ends_with(']')) {
        return Some(ListElement::Number(input.parse().unwrap()));
    }

    let slice = &input[1..input.len() - 1];

    let mut res = vec![];
    let mut it = slice.chars();
    let mut buf = String::new();
    let mut index = 0;
    loop {
        match it.next() {
            Some(c) => match c {
                ',' => {
                    if index == 0 {
                        if let Some(parsed) = parse_input_list(&buf) {
                            res.push(parsed);
                        }
                        buf.clear();
                    } else {
                        buf.push(c);
                    }
                }
                '[' => {
                    index += 1;
                    buf.push(c);
                }
                ']' => {
                    index -= 1;
                    buf.push(c);
                }
                _ => buf.push(c),
            },
            None => break,
        }
    }

    if let Some(parsed) = parse_input_list(&buf) {
        res.push(parsed);
    }
    Some(ListElement::List(res))
}

fn parse_input(input: &str) -> Vec<(ListElement, ListElement)> {
    input
        .split("\n\n")
        .map(|l| {
            let (a, b) = l.split_once('\n').unwrap();
            (
                parse_input_list(a.trim()).unwrap(),
                parse_input_list(b.trim()).unwrap(),
            )
        })
        .collect()
}

pub fn puzzle_1(input: &str) -> String {
    let pairs = parse_input(input);

    let sum: usize = pairs
        .iter()
        .map(|(left, right)| left <= right)
        .enumerate()
        .filter_map(|(id, result)| match result {
            true => Some(id + 1),
            false => None,
        })
        .sum();

    sum.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input = parse_input(input);
    let mut packets = input
        .into_iter()
        .flat_map(|(a, b)| vec![a, b])
        .collect::<BTreeSet<_>>();
    let divider_1 = parse_input_list("[[2]]").unwrap();
    let divider_2 = parse_input_list("[[6]]").unwrap();

    packets.insert(divider_1.clone());
    packets.insert(divider_2.clone());

    let index_1 = packets.iter().position(|el| *el == divider_1).unwrap() + 1;
    let index_2 = packets.iter().position(|el| *el == divider_2).unwrap() + 1;

    (index_1 * index_2).to_string()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    use super::*;

    #[test]
    fn test_can_parse_list() {
        use ListElement::*;

        let input = "[[4,4],4,4]";
        let result = parse_input_list(input);
        assert_eq!(
            result,
            Some(List(vec![
                List(vec![Number(4), Number(4)]),
                Number(4),
                Number(4)
            ]))
        );

        let input = "[]";
        let result = parse_input_list(input);
        assert_eq!(result, Some(List(vec![])));
    }

    #[test]
    fn test_can_parse_input() {
        let result = parse_input(INPUT);

        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_orderings() {
        let input = "[1,1,3,1,1]\n[1,1,5,1,1]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a < b);

        let input = "[[1],[2,3,4]]\n[[1],4]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a < b);

        let input = "[9]\n[[8,7,6]]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a > b);

        let input = "[[4,4],4,4]\n[[4,4],4,4,4]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a < b);

        let input = "[7,7,7,7]\n[7,7,7]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a > b);

        let input = "[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a > b);
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "13");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "140");
    }
}
