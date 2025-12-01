#[derive(Debug)]
struct Input {
    distance: i16,
}

fn load_input(input: &str) -> Vec<Input> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|l| l.split_at(1))
        .map(|(direction, distance)| {
            let direction = match direction.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!("Unhandled"),
            };
            let distance = distance.parse::<i16>().unwrap();
            Input {
                distance: direction * distance,
            }
        })
        .collect()
}

pub fn puzzle_1(input: &str) -> String {
    let input = load_input(input);
    let mut dial = 50;
    let mut res = 0;
    for code in input {
        dial += code.distance;
        if dial % 100 == 0 {
            res += 1;
        }
    }

    res.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input = load_input(input);
    let mut dial = 50;
    let mut res = 0;
    for code in input {
        let range = match code.distance > 0 {
            true => dial + 1..dial + code.distance + 1,
            false => dial + code.distance..dial,
        };
        for number in range {
            if number % 100 == 0 {
                res += 1;
            }
        }
        dial += code.distance;
        dial %= 100;
    }

    res.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "3");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "6");
    }
}
