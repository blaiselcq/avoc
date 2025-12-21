use core::str;

fn parse_input(input: &str) -> (u16, Vec<u16>) {
    let len = input.split_once('\n').unwrap().0.len() as u16;
    let numbers = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| u16::from_str_radix(l, 2).unwrap())
        .collect();

    (len, numbers)
}

fn get_most_common_nth_bit(bits: &[u16], k: u16) -> u16 {
    let counts = bits
        .iter()
        .map(|&n| (n & (1 << k)) >> k)
        .fold((0, 0), |acc, b| match b {
            0 => (acc.0 + 1, acc.1),
            1 => (acc.0, acc.1 + 1),
            _ => unreachable!(),
        });
    if counts.0 > counts.1 {
        0
    } else {
        1
    }
}

fn get_least_common_nth_bit(bits: &Vec<u16>, k: u16) -> u16 {
    get_most_common_nth_bit(bits, k) ^ 1
}

fn filter_on_most_common_nth_bit(bits: &Vec<u16>, k: u16) -> Vec<u16> {
    if bits.len() == 1 {
        return bits.clone();
    }

    let mcnb = get_most_common_nth_bit(bits, k);
    bits.iter()
        .filter(|&n| (n & (1 << k)) >> k == mcnb)
        .map(|&n| n)
        .collect()
}

fn filter_on_least_common_nth_bit(bits: &Vec<u16>, k: u16) -> Vec<u16> {
    if bits.len() == 1 {
        return bits.clone();
    }

    let lcnb = get_least_common_nth_bit(bits, k);
    bits.iter()
        .filter(|&n| (n & (1 << k)) >> k == lcnb)
        .map(|&n| n)
        .collect()
}

pub fn puzzle_1(input: &str) -> String {
    let (len, input) = parse_input(input);
    let gamma_rate = (0..len)
        .map(|i| get_most_common_nth_bit(&input, i) * 1 << i)
        .reduce(|acc, e| acc | e)
        .unwrap() as u32;
    let epsilon_rate = (0..len)
        .map(|i| get_least_common_nth_bit(&input, i) * 1 << i)
        .reduce(|acc, e| acc | e)
        .unwrap() as u32;

    (gamma_rate * epsilon_rate).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (len, input) = parse_input(input);

    let oo_rating = *(0..len)
        .rev()
        .fold(input.clone(), |acc, n| {
            filter_on_most_common_nth_bit(&acc, n)
        })
        .first()
        .unwrap() as u32;

    let coo_rating = *(0..len)
        .rev()
        .fold(input, |acc, n| filter_on_least_common_nth_bit(&acc, n))
        .first()
        .unwrap() as u32;

    (oo_rating * coo_rating).to_string()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        let result = puzzle_1(INPUT);

        assert_eq!(result, "198");
    }
    #[test]
    fn test_puzzle_2() {
        let result = puzzle_2(INPUT);

        assert_eq!(result, "230");
    }
}
