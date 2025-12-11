use rayon::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u32>,
}

fn load_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let mut blocks = line.split(' ');
            let lights = blocks.next().unwrap();
            let buttons = blocks.take_while(|s| s.chars().next().unwrap() == '(');
            let joltage = line.split(' ').last().unwrap();

            let lights = lights
                .trim_matches(|c| c == '[' || c == ']')
                .chars()
                .map(|c| c == '#')
                .collect::<Vec<_>>();

            let buttons = buttons
                .map(|button| {
                    button
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|c| c.parse().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let joltage = joltage
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<_>>();

            Machine {
                lights,
                buttons,
                joltage,
            }
        })
        .collect()
}

fn press_buttons_for_lights(mut lights: Vec<bool>, button: &[usize]) -> Vec<bool> {
    for &i in button {
        lights[i] = !lights[i];
    }

    lights
}

fn press_buttons_for_joltage(mut joltage: Vec<u32>, button: &[usize], presses: u32) -> Vec<u32> {
    for &i in button {
        joltage[i] += presses;
    }

    joltage
}

fn search_buttons_for_lights(machine: &Machine) -> usize {
    let mut states = BTreeMap::from([(0usize, vec![vec![false; machine.lights.len()]])]);
    let min_presses = loop {
        let mut entry = states.first_entry().unwrap();
        let presses = *entry.key();
        let state = entry.get_mut().pop().unwrap();

        if entry.get().is_empty() {
            states.remove(&presses);
        }

        if state == machine.lights {
            break presses;
        }

        let next_press_entry = states.entry(presses + 1).or_default();
        next_press_entry.extend(
            machine
                .buttons
                .iter()
                .map(|button| press_buttons_for_lights(state.clone(), button)),
        );
    };

    min_presses
}

fn joltage_all_buttons(machine: &Machine) -> Vec<u32> {
    let mut joltage = vec![0; machine.joltage.len()];
    for button in &machine.buttons {
        joltage = press_buttons_for_joltage(joltage, button, 1);
    }
    joltage
}

fn search_buttons_for_joltage(machine: &Machine) -> u32 {
    let total_joltage: u32 = machine.joltage.iter().sum();

    let mut states = BTreeMap::from([(0u32, vec![(0u32, vec![0; machine.joltage.len()])])]);

    let range = (0..=machine.joltage.iter().max().unwrap().ilog2()).map(|n| 2u32.pow(n));
    let min_presses = loop {
        let mut entry = states.first_entry().unwrap();
        let cost = *entry.key();
        let (presses, state) = entry.get_mut().pop().unwrap();

        if entry.get().is_empty() {
            states.remove(&cost);
        }

        if state == machine.joltage {
            break presses;
        }

        for button in &machine.buttons {
            for p in range.clone() {
                let new_joltage = press_buttons_for_joltage(state.clone(), button, p);
                if machine.joltage.iter().zip(&new_joltage).any(|(m, n)| n > m) {
                    continue;
                }
                let cost = presses + p + total_joltage - new_joltage.iter().sum::<u32>();
                states
                    .entry(cost)
                    .or_default()
                    .push((presses + p, new_joltage));
            }
        }
    };

    min_presses
}

pub fn puzzle_1(input: &str) -> String {
    let input = load_input(input);

    let res: usize = input
        .iter()
        .map(|machine| search_buttons_for_lights(machine))
        .sum();

    res.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input = load_input(input);

    let mut i = 0;
    let res: u32 = input
        .iter()
        .inspect(|_| {
            println!("{}", i);
            i += 1;
        })
        .map(|machine| search_buttons_for_joltage(machine))
        .sum();

    res.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "7");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "33");
    }
}
