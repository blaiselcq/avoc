use core::panic;
use std::{
    env,
    io::{self, Read},
    time::Instant,
};

use days::get_days;
use utils::Day;

mod days;
mod utils;

fn execute(year: u16, day: &Day, puzzle_number: u8, input: &String) {
    let start = Instant::now();
    let puzzle = match puzzle_number {
        1 => day.puzzle_1,
        2 => day.puzzle_2,
        _ => panic!(),
    };

    let result = puzzle(input);

    println!(
        "Year {:04} \t Day {:02} \t Time: {:.2e} s \t Puzzle {}: {}",
        year,
        day.day,
        start.elapsed().as_secs_f32(),
        puzzle_number,
        result
    );
}

fn get_input() -> io::Result<String> {
    let mut res = String::new();
    io::stdin().lock().read_to_string(&mut res)?;
    Ok(res)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = get_input().expect("Failed to parse input");
    let year = args
        .get(1)
        .and_then(|d| str::parse::<u16>(d).ok())
        .expect("Expected a year number");
    let days = get_days(year);
    let day_number = args
        .get(2)
        .and_then(|d| str::parse::<usize>(d).ok())
        .expect("Expected a day number");
    let selected_day = days
        .get(day_number - 1)
        .expect("Cannot find selected day for selected year");
    let puzzle_number = args.get(3).and_then(|p| str::parse::<usize>(p).ok());

    if puzzle_number.is_none() || puzzle_number == Some(1) {
        execute(year, selected_day, 1, &input);
    }
    if puzzle_number.is_none() || puzzle_number == Some(2) {
        execute(year, selected_day, 2, &input);
    }
}
