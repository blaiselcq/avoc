fn load_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn largest_joltage(bank: &[u32], batteries: usize) -> u64 {
    let mut res = 0u64;
    let mut start_pos = 0;
    for i in (0..batteries).rev() {
        let digit = *(bank[start_pos..bank.len() - i].iter().max().unwrap());
        let pos_digit = bank[start_pos..bank.len() - i]
            .iter()
            .position(|&d| d == digit)
            .unwrap();
        res = 10 * res + digit as u64;
        start_pos += pos_digit + 1;
    }

    res
}

pub fn puzzle_1(input: &str) -> String {
    let input = load_input(input);
    let res: u64 = input.iter().map(|bank| largest_joltage(bank, 2)).sum();
    res.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input = load_input(input);
    let res: u64 = input.iter().map(|bank| largest_joltage(bank, 12)).sum();
    res.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "357");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "3121910778619");
    }
}
