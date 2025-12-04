use crate::utils::{get_day, Day};

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;

pub fn get_days() -> Vec<Day> {
    vec![
        get_day!(1, day_01),
        get_day!(2, day_02),
        get_day!(3, day_03),
        get_day!(4, day_04),
    ]
}
