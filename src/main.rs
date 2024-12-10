use std::time::Instant;

use day01::day1;
use day02::day2;
use day03::day3;
use day04::day4;
use day05::day5;
use day06::day6;
use day07::day7;
use day08::day8;
use day09::day9;
use day10::day10;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    time_run(day1);
    time_run(day2);
    time_run(day3);
    time_run(day4);
    time_run(day5);
    time_run(day6);
    time_run(day7);
    time_run(day8);
    time_run(day9);
    time_run(day10);
}

fn time_run(f: impl Fn()) {
    let before = Instant::now();
    f();
    let after = Instant::now();
    let duration = (after - before).as_millis();
    println!("{}ms", duration);
    println!();
}
