use std::{
    collections::{hash_map::DefaultHasher, BTreeSet, HashMap},
    hash::{Hash, Hasher},
};

type ValveName = (char, char);

pub(crate) struct InputData {
    paths: HashMap<ValveName, (Vec<ValveName>, u16)>,
    flow_rate: Vec<(ValveName, u16)>,
}

#[derive(Hash, Clone)]
struct ValvesConfiguration {
    data: [[bool; 26]; 26],
}

impl ValvesConfiguration {
    fn new() -> Self {
        Self {
            data: [[false; 26]; 26],
        }
    }

    fn is_open(&self, valve_name: ValveName) -> bool {
        let a_val = 'A' as usize;

        let id_1 = valve_name.0 as usize - a_val;
        let id_2 = valve_name.1 as usize - a_val;

        self.data[id_1][id_2]
    }

    fn open(&mut self, valve_name: &ValveName) {
        let a_val = 'A' as usize;

        let id_1 = valve_name.0 as usize - a_val;
        let id_2 = valve_name.1 as usize - a_val;

        self.data[id_1][id_2] = true;
    }
}

struct Node {
    human_valve: ValveName,
    elephant_valve: ValveName,
    released_pressure: u16,
    minute: u8,
    configuration: ValvesConfiguration,
}

struct NodeContainer {
    nodes: Vec<Node>,
    visited_nodes: BTreeSet<u64>,
}

impl NodeContainer {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            visited_nodes: BTreeSet::new(),
        }
    }

    fn insert(&mut self, node: Node) {
        let mut hasher = DefaultHasher::new();
        node.hash(&mut hasher);
        if self.visited_nodes.insert(hasher.finish()) {
            self.nodes.push(node);
        }
    }

    fn pop(&mut self) -> Option<Node> {
        self.nodes.pop()
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.human_valve.hash(state);
        self.elephant_valve.hash(state);
        self.released_pressure.hash(state);
        self.minute.hash(state);
        // self.configuration.hash(state);
    }
}

fn parse_valve_name(input: &str) -> ValveName {
    let mut input = input.chars();
    (input.next().unwrap(), input.next().unwrap())
}

fn parse_input(input: &str) -> InputData {
    // A line will look like that:
    //   Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    let parsed = input.lines().map(|line| {
        let (valve_line, tunnel_line) = line.split_once("; ").unwrap();
        let (valve_str, valve_line) = valve_line
            .strip_prefix("Valve ")
            .unwrap()
            .split_once(' ')
            .unwrap();
        let valve = parse_valve_name(valve_str);
        let flow_rate = valve_line.split_once('=').unwrap().1.parse().unwrap();

        let tunnels = tunnel_line
            .split_once("to ")
            .unwrap()
            .1
            .split_once(' ')
            .unwrap()
            .1
            .split(' ')
            .map(|v| parse_valve_name(v))
            .collect();

        (valve, (flow_rate, tunnels))
    });

    let flow_rate = parsed
        .clone()
        .map(|(name, (flow_rate, _))| (name, flow_rate))
        .collect();

    let paths = parsed
        .map(|(name, (flow_rate, paths))| (name, (paths, flow_rate)))
        .collect();

    InputData { paths, flow_rate }
}

fn get_released_pressure(node: &Node, input_data: &InputData) -> u16 {
    input_data.flow_rate.iter().fold(0, |acc, (key, val)| {
        match node.configuration.is_open(*key) {
            true => acc + val,
            false => acc,
        }
    })
}

fn get_starting_node() -> Node {
    let configuration = ValvesConfiguration::new();

    let initial_valve = ('A', 'A');

    Node {
        human_valve: initial_valve,
        elephant_valve: initial_valve,
        released_pressure: 0,
        minute: 0,
        configuration,
    }
}

fn human_opens(
    node: &Node,
    nodes: &mut NodeContainer,
    input_data: &InputData,
    use_elephants: bool,
) {
    let path_elephants: Box<dyn Iterator<Item = &ValveName>> = match use_elephants {
        true => Box::new(input_data.paths.get(&node.elephant_valve).unwrap().0.iter()),
        false => Box::new(std::iter::once(&node.elephant_valve)),
    };

    path_elephants
        .filter(|path| {
            !(node.configuration.is_open(node.human_valve) && node.configuration.is_open(**path))
        })
        .for_each(|path| {
            let mut new_configuration = node.configuration.clone();
            new_configuration.open(&node.human_valve);

            let released_pressure = get_released_pressure(node, input_data);

            let new_node = Node {
                human_valve: node.human_valve,
                elephant_valve: *path,
                released_pressure: node.released_pressure + released_pressure,
                minute: node.minute + 1,
                configuration: new_configuration,
            };

            nodes.insert(new_node);
        });
}

fn elephant_opens(
    node: &Node,
    nodes: &mut NodeContainer,
    input_data: &InputData,
    _use_elephants: bool,
) {
    input_data
        .paths
        .get(&node.human_valve)
        .unwrap()
        .0
        .iter()
        .filter(|path| {
            !(node.configuration.is_open(node.elephant_valve) && node.configuration.is_open(**path))
        })
        .for_each(|path| {
            let mut new_configuration = node.configuration.clone();
            new_configuration.open(&node.elephant_valve);

            let released_pressure = get_released_pressure(node, input_data);

            let new_node = Node {
                human_valve: *path,
                elephant_valve: node.elephant_valve,
                released_pressure: node.released_pressure + released_pressure,
                minute: node.minute + 1,
                configuration: new_configuration,
            };

            nodes.insert(new_node);
        });
}

fn both_opens(node: &Node, nodes: &mut NodeContainer, input_data: &InputData, use_elephants: bool) {
    let mut new_configuration = node.configuration.clone();
    if use_elephants {
        new_configuration.open(&node.elephant_valve);
    }
    new_configuration.open(&node.human_valve);

    let released_pressure = get_released_pressure(node, input_data);

    let new_node = Node {
        human_valve: node.human_valve,
        elephant_valve: node.elephant_valve,
        released_pressure: node.released_pressure + released_pressure,
        minute: node.minute + 1,
        configuration: new_configuration,
    };

    nodes.insert(new_node);
}

fn both_moves(node: &Node, nodes: &mut NodeContainer, input_data: &InputData, use_elephants: bool) {
    let path_elephants: Box<dyn Iterator<Item = &ValveName>> = match use_elephants {
        true => Box::new(input_data.paths.get(&node.elephant_valve).unwrap().0.iter()),
        false => Box::new(std::iter::once(&node.elephant_valve)),
    };

    path_elephants.for_each(|path_elephant| {
        let path_humans = input_data
            .paths
            .get(&node.human_valve)
            .unwrap()
            .0
            .iter()
            .filter(|path_human| {
                !(node.configuration.is_open(*path_elephant)
                    && node.configuration.is_open(**path_human))
            });
        path_humans.for_each(|path_human| {
            let released_pressure = get_released_pressure(node, input_data);

            let new_node = Node {
                human_valve: *path_human,
                elephant_valve: *path_elephant,
                released_pressure: node.released_pressure + released_pressure,
                minute: node.minute + 1,
                configuration: node.configuration.clone(),
            };

            nodes.insert(new_node);
        });
    });
}

fn solve(input_data: &InputData, max_minute: u8, use_elephants: bool) -> u16 {
    let mut nodes = NodeContainer::new();

    nodes.insert(get_starting_node());

    let mut max_flow_rate = 0;

    while let Some(node) = nodes.pop() {
        max_flow_rate = max_flow_rate.max(node.released_pressure);

        if node.minute == max_minute {
            continue;
        };

        let can_human_open = !node.configuration.is_open(node.human_valve)
            && input_data.paths.get(&node.human_valve).unwrap().1 > 0;
        let can_elephant_open = !node.configuration.is_open(node.elephant_valve)
            && input_data.paths.get(&node.elephant_valve).unwrap().1 > 0;

        if use_elephants && can_elephant_open && can_human_open {
            both_opens(&node, &mut nodes, input_data, use_elephants);
        }
        if can_human_open {
            human_opens(&node, &mut nodes, input_data, use_elephants);
        }
        if use_elephants && can_elephant_open {
            elephant_opens(&node, &mut nodes, input_data, use_elephants);
        }
        both_moves(&node, &mut nodes, input_data, use_elephants);
    }

    max_flow_rate
}

pub fn puzzle_1(input: &str) -> String {
    let input_data = parse_input(input);

    solve(&input_data, 30, false).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input_data = parse_input(input);

    // This bruteforce solution takes a long time, TODO: find a more clever solution

    solve(&input_data, 26, true).to_string()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    use super::*;

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(INPUT);
        let paths = parsed.paths.get(&('A', 'A')).unwrap();
        assert_eq!(paths, &(vec![('D', 'D'), ('I', 'I'), ('B', 'B')], 0),);
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "1651");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "1707");
    }
}
