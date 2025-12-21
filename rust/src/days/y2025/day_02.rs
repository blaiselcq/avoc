fn load_input(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .split(',')
        .map(|str| str.split_once('-').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect()
}

fn is_valid_n(number: i64) -> bool {
    let number = number.to_string().chars().collect::<Vec<char>>();

    for i in 1..number.len() {
        if number.len() % i != 0 {
            continue;
        }
        if number.chunks(i).all(|c| c == &number[..i]) {
            return false;
        }
    }

    true
}

fn is_valid_1(number: i64) -> bool {
    let number = number.to_string();

    if number.len() % 2 == 1 {
        return true;
    }

    let (l, r) = number.split_at(number.len() / 2);

    l != r
}

pub fn puzzle_1(input: &str) -> String {
    let input = load_input(input);
    let mut res: i64 = 0;

    for (l, r) in input {
        for i in l..=r {
            if !is_valid_1(i) {
                res += i;
            }
        }
    }
    res.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input = load_input(input);
    let mut res: i64 = 0;

    for (l, r) in input {
        for i in l..=r {
            if !is_valid_n(i) {
                res += i;
            }
        }
    }
    res.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "1227775554");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "4174379265");
    }
}
