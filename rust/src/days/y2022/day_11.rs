mod types {
    use std::collections::VecDeque;

    pub type Operation = dyn Fn(u64) -> Option<u64>;
    pub struct Monkey {
        pub items: VecDeque<u64>,
        pub operation: Box<Operation>,
        pub test_number: u64,
        pub recipient_success: usize,
        pub recipient_failed: usize,
    }

    impl Monkey {
        pub fn play(&mut self, managed_worry: bool, divisor: u64) -> Option<(usize, u64)> {
            let value = self.items.pop_front()?;

            let worry_level = match managed_worry {
                false => (self.operation)(value % divisor)?,
                true => (self.operation)(value)? / 3,
            };

            match worry_level % self.test_number == 0 {
                true => Some((self.recipient_success, worry_level)),
                false => Some((self.recipient_failed, worry_level)),
            }
        }

        pub fn throw(&mut self, value: u64) {
            self.items.push_back(value);
        }
    }
}

mod parser {
    use core::panic;
    use std::num::ParseIntError;

    pub use super::types::Monkey;

    #[derive(Debug, Clone, Copy)]
    enum Operand {
        Old,
        Number(u64),
    }

    fn map_operand(operand: &str) -> Result<Operand, ParseIntError> {
        match operand {
            "old" => Ok(Operand::Old),
            string => {
                let number = string.parse()?;
                Ok(Operand::Number(number))
            }
        }
    }

    fn parse_operation(input: &str) -> Box<super::types::Operation> {
        let input = input.strip_prefix(" new = ").unwrap();
        let mut input = input.split(' ').map(|x| x.trim());

        let operand_1_str = input.next().unwrap();
        let operator_str = input.next().unwrap();
        let operand_2_str = input.next().unwrap();

        let operand_1 = map_operand(operand_1_str).unwrap();
        let operand_2 = map_operand(operand_2_str).unwrap();

        let operator = match operator_str.chars().next().unwrap() {
            '+' => u64::checked_add,
            '*' => u64::checked_mul,
            _ => panic!(),
        };

        let operation: Box<dyn Fn(u64) -> Option<u64>> = match (operand_1, operand_2) {
            (Operand::Old, Operand::Old) => Box::new(move |old| operator(old, old)),
            (Operand::Old, Operand::Number(n2)) => Box::new(move |old| operator(old, n2)),
            (Operand::Number(n1), Operand::Old) => Box::new(move |old| operator(n1, old)),
            (Operand::Number(n1), Operand::Number(n2)) => Box::new(move |_| operator(n1, n2)),
        };

        operation
    }

    fn get_last_number(input: &str) -> usize {
        input.rsplit_once(' ').unwrap().1.parse().unwrap()
    }

    fn parse_monkey(input: &str) -> Monkey {
        let mut lines = input.lines();
        lines.next(); // monkey line
        let items = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|i| i.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let operation =
            parse_operation(lines.next().unwrap().strip_prefix("  Operation:").unwrap());
        let test_number = get_last_number(lines.next().unwrap()) as u64;
        let recipient_success = get_last_number(lines.next().unwrap());
        let recipient_failed = get_last_number(lines.next().unwrap());

        Monkey {
            items: items.into(),
            operation,
            test_number,
            recipient_success,
            recipient_failed,
        }
    }

    pub fn parse_input(input: &str) -> Vec<Monkey> {
        input.split("\n\n").map(|l| parse_monkey(l)).collect()
    }

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0";

        let monkey = parse_monkey(input);

        assert_eq!(monkey.items, vec![54, 65, 75, 74]);
        assert_eq!(monkey.test_number, 19);
        assert_eq!(monkey.recipient_success, 2);
        assert_eq!(monkey.recipient_failed, 0);
    }
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn monkey_play(
    iterations: usize,
    mut monkeys: Vec<types::Monkey>,
    managed_worry: bool,
) -> Vec<usize> {
    let mut pass = vec![0; monkeys.len()];

    let divisor = match managed_worry {
        true => 3,
        false => lcm(&monkeys.iter().map(|m| m.test_number).collect::<Vec<_>>()),
    };

    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            while let Some(play) = monkeys[i].play(managed_worry, divisor) {
                let (recipient, value) = play;
                monkeys[recipient].throw(value);
                pass[i] += 1;
            }
        }
    }

    pass
}

pub fn puzzle_1(input: &str) -> String {
    let monkeys = parser::parse_input(input);
    let mut pass = monkey_play(20, monkeys, true);
    pass.sort();

    (pass.pop().unwrap() * pass.pop().unwrap()).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let monkeys = parser::parse_input(input);
    let mut pass = monkey_play(10000, monkeys, false);

    pass.sort();

    (pass.pop().unwrap() * pass.pop().unwrap()).to_string()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "10605");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "2713310158");
    }
}
