use core::panic;
use std::iter::once;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
}

fn load_input(input: &str) -> Vec<(Operator, Vec<String>)> {
    let indexes = input
        .lines()
        .next_back()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|&(_i, c)| c == '+' || c == '*')
        .map(|(i, c)| {
            (
                i,
                match c {
                    '*' => Operator::Mul,
                    '+' => Operator::Add,
                    _ => panic!(),
                },
            )
        });

    let mut result = indexes
        .clone()
        .map(|(_i, c)| (c, Vec::<String>::new()))
        .collect::<Vec<_>>();

    let split = indexes
        .into_iter()
        .skip(1)
        .map(|(i, _c)| i)
        .chain(once(input.lines().next().unwrap().len() + 1));

    input
        .lines()
        .take_while(|l| !(l.contains('*') || l.contains('+')))
        .for_each(|l| {
            let mut r = result.iter_mut();
            let mut last_i = 0;
            for i in split.clone() {
                r.next().unwrap().1.push(l[last_i..i - 1].to_string());
                last_i = i;
            }
        });

    result
}

fn apply(operation: Operator, operands: &[usize]) -> usize {
    operands.iter().fold(
        match operation {
            Operator::Add => 0,
            Operator::Mul => 1,
        },
        |acc, s| match operation {
            Operator::Add => acc + s,
            Operator::Mul => acc * s,
        },
    )
}

fn to_normal_math(operands: &[String]) -> Vec<usize> {
    operands.iter().map(|s| s.trim().parse().unwrap()).collect()
}

fn to_cephalopods_math(operands: &[String]) -> Vec<usize> {
    let max_len = operands.iter().map(|s| s.len()).max().unwrap();
    let mut cephalopods_numbers = vec![];
    cephalopods_numbers.resize(max_len, String::new());

    operands.iter().for_each(|str| {
        str.chars()
            .enumerate()
            .for_each(|(i, c)| cephalopods_numbers.get_mut(i).unwrap().push(c));
    });
    cephalopods_numbers
        .iter()
        .map(|str| str.trim().parse().unwrap())
        .collect()
}

pub fn puzzle_1(input: &str) -> String {
    let res = load_input(input)
        .iter()
        .map(|(operator, operands)| {
            let operands = to_normal_math(operands);
            apply(*operator, &operands)
        })
        .sum::<usize>();

    res.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let res = load_input(input)
        .iter()
        .map(|(operator, operands)| {
            let operands = to_cephalopods_math(operands);
            apply(*operator, &operands)
        })
        .sum::<usize>();

    res.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "4277556");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "3263827");
    }
}
