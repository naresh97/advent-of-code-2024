use std::time::Instant;

use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    time_run(day1);
    time_run(day2);
    time_run(day3);
    time_run(day4);
    time_run(day5);
    time_run(day6);
    time_run(day7);
}

fn time_run(f: impl Fn()) {
    let before = Instant::now();
    f();
    let after = Instant::now();
    let duration = (after - before).as_millis();
    println!("{}ms", duration);
    println!();
}
