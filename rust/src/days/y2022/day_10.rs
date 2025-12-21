use std::collections::BTreeMap;

use nom::character::complete::{alpha1, newline};
use nom::character::streaming::space1;
use nom::error::{Error, ErrorKind};
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct Cpu {
    counter: usize,
    register_x: i32,
}

impl Instruction {
    fn parser(input: &str) -> IResult<&str, Instruction> {
        let (input, instruction) = alpha1(input)?;
        match instruction {
            "noop" => Ok((input, Instruction::Noop)),
            "addx" => {
                let (input, (_, x)) = tuple((space1, nom::character::complete::i32))(input)?;
                Ok((input, Instruction::Addx(x)))
            }
            other => IResult::Err(nom::Err::Failure(Error::new(other, ErrorKind::Switch))),
        }
    }
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            counter: 0,
            register_x: 1,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => self.cycle(),
            Instruction::Addx(x) => {
                self.cycle();
                self.cycle();
                self.register_x += x;
            }
        }
    }

    fn cycle(&mut self) {
        self.counter += 1;
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let (_, output) = separated_list0(newline, Instruction::parser)(input).unwrap();
    output
}

fn complete_history(history: BTreeMap<usize, i32>) -> Vec<i32> {
    let mut output = vec![];
    let max_keys = history.keys().max();
    let min_keys = history.keys().min();
    if max_keys.is_none() || min_keys.is_none() {
        return output;
    }
    let max_keys = max_keys.unwrap();

    for i in 0..=*max_keys {
        if &i < min_keys.unwrap() {
            output.push(0);
            continue;
        }
        let value = history
            .iter()
            .filter(|(&k, _)| k <= i)
            .max_by_key(|(&k, _)| k);
        if let Some(value) = value {
            output.push(*value.1);
        }
    }
    output
}

fn get_signal_strengths(start: usize, increment: usize, history: BTreeMap<usize, i32>) -> Vec<i32> {
    let increment_selector = |key: usize| {
        if key < start.saturating_sub(1) {
            return false;
        }
        (key + 1 - start) % increment == 0
    };

    let history = complete_history(history);
    history[..history.len() - 1]
        .iter()
        .enumerate()
        .filter(|&(key, _)| increment_selector(key))
        .map(|(key, value)| ((key + 1) as i32, value))
        .map(|(key, value)| key * value)
        .collect()
}

fn get_drawing(length: usize, history: BTreeMap<usize, i32>) -> Vec<Vec<bool>> {
    let history = complete_history(history)
        .iter()
        .enumerate()
        .map(|(i, v)| (*v - (i % length) as i32).abs() < 2)
        .collect::<Vec<_>>();
    history
        .chunks(length)
        .map(|c| c.iter().cloned().collect())
        .collect()
}

fn format_drawing(drawing: Vec<Vec<bool>>) -> String {
    let mut output = String::new();
    drawing.iter().for_each(|line| {
        let mut out = String::new();
        line.iter().for_each(|c| match c {
            true => out.push('#'),
            false => out.push(' '),
        });
        output.push('\n');
        output.push_str(&out);
    });

    output
}

pub fn puzzle_1(input: &str) -> String {
    let instructions = parse_input(input);

    let mut history = BTreeMap::new();

    let mut cpu = Cpu::new();
    cpu.register_x = 1;
    history.insert(cpu.counter, cpu.register_x);
    instructions.iter().for_each(|&instruction| {
        cpu.execute(instruction);
        history.insert(cpu.counter, cpu.register_x);
    });

    let strength: i32 = get_signal_strengths(20, 40, history).iter().sum();

    strength.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let instructions = parse_input(input);

    let mut history = BTreeMap::new();

    let mut cpu = Cpu::new();
    history.insert(cpu.counter, cpu.register_x);
    instructions.iter().for_each(|&instruction| {
        cpu.execute(instruction);
        history.insert(cpu.counter, cpu.register_x);
    });
    let drawing = get_drawing(40, history);
    format_drawing(drawing)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    use super::*;

    #[test]
    fn test_parse_instruction() {
        let instructions = "noop\naddx 3\naddx -5";
        let instructions = parse_input(instructions);
        assert_eq!(
            instructions,
            vec![
                Instruction::Noop,
                Instruction::Addx(3),
                Instruction::Addx(-5)
            ]
        );
    }

    #[test]
    fn test_get_history() {
        let mut history = BTreeMap::new();
        history.insert(0, 1);
        history.insert(2, 3);
        history.insert(3, 4);
        history.insert(6, 8);

        assert_eq!(complete_history(history), vec![1, 1, 3, 4, 4, 4, 8]);
    }

    #[test]
    fn test_get_signal_strength() {
        let mut history = BTreeMap::new();
        history.insert(19, 3);
        history.insert(21, 5);
        history.insert(40, 9);
        history.insert(59, 23);

        assert_eq!(get_signal_strengths(0, 20, history), vec![3 * 20, 40 * 5]);
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "13140");
    }
}
