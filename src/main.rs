use std::env;

mod day_01;
mod day_02a;
mod day_02b;
mod day_03a;

fn main() {
    let day = env::args().nth(1).expect("invalid 'day' argument");

    let f = match day.as_str() {
        "1" => day_01::run,
        "2a" => day_02a::run,
        "2b" => day_02b::run,
        "3a" => day_03a::run,
        _ => panic!("day not found: {:?}", day),
    };

    let _ = f();
}
