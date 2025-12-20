#[allow(dead_code)]
pub enum InputKind {
    Test,
    Run,
}

pub struct Day {
    pub day: u8,
    pub puzzle_1: fn(&str) -> String,
    pub puzzle_2: fn(&str) -> String,
}

macro_rules! get_day {
    ($n: tt, $mod:tt) => {
        Day {
            day: $n,
            puzzle_1: $mod::puzzle_1,
            puzzle_2: $mod::puzzle_2,
        }
    };
}
pub(crate) use get_day;
