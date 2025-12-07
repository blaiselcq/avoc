fn load_input(input: &str) -> (usize, impl Iterator<Item = Vec<usize>> + use<'_>) {
    (
        input
            .lines()
            .next()
            .unwrap()
            .chars()
            .position(|c| c == 'S')
            .unwrap(),
        input.lines().skip(1).map(|l| {
            l.char_indices()
                .filter_map(|(i, c)| match c {
                    '^' => Some(i),
                    _ => None,
                })
                .collect()
        }),
    )
}

pub fn puzzle_1(input: &str) -> String {
    let (start, splitters) = load_input(input);

    let width = input.lines().next().unwrap().len();
    let rays = {
        let mut res = vec![false; width];
        res[start] = true;
        res
    };

    let res: usize = splitters
        .scan(rays, |rays, splitters| {
            let mut count = 0;
            let mut new_rays = vec![false; width];
            for (i, is_present) in rays.iter().enumerate() {
                if *is_present {
                    if splitters.contains(&i) {
                        new_rays[i - 1] = true;
                        new_rays[i + 1] = true;
                        count += 1
                    } else {
                        new_rays[i] = true;
                    }
                }
            }
            *rays = new_rays;
            Some(count)
        })
        .sum();

    res.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (start, splitters) = load_input(input);

    let width = input.lines().next().unwrap().len();
    let rays = {
        let mut res = vec![0usize; width];
        res[start] = 1;
        res
    };

    let rays = splitters.fold(rays, |rays, splitters| {
        let mut new_rays = vec![0usize; width];
        for (i, ray_count) in rays.iter().enumerate() {
            if splitters.contains(&i) {
                new_rays[i - 1] += ray_count;
                new_rays[i + 1] += ray_count;
            } else {
                new_rays[i] += ray_count;
            }
        }
        new_rays
    });

    let res: usize = rays.iter().sum();
    res.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "21");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "40");
    }
}
