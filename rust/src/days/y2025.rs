use crate::utils::{get_day, Day};

pub mod day_01;

pub fn get_days() -> Vec<Day> {
    vec![get_day!(1, day_01)]
}
