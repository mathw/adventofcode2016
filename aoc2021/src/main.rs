use crate::day::Day;
use crate::day::DayResult;
use clap::{App, Arg};
use std::error::Error;
use std::time::Instant;

#[macro_use]
extern crate lazy_static;

mod bingo;
mod common;
mod day;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let matches = App::new("Advent of Code 2021")
        .version("1.0")
        .author("Matthew Walton")
        .about("Solves Advent of Code 2021 problems")
        .arg(
            Arg::with_name("DAY")
                .help("Chooses which day to run")
                .required(true)
                .index(1),
        )
        .get_matches();

    let day = matches.value_of("DAY").expect("Day must be provided");

    match day {
        "1" => run_day(1, || {
            let mut day = day1::Day1::new();
            day.run()
        }),
        "2" => run_day(2, || day2::run()),
        "3" => run_day(3, || day3::run()),
        "4" => run_day(4, || day4::run()),
        "5" => run_day(5, || day5::run()),
        "6" => run_day(6, || day6::run()),
        "7" => run_day(7, || day7::run()),
        "8" => run_day(8, || day8::run()),
        "9" => run_day(9, || day9::run()),
        "10" => run_day(10, || day10::run()),
        "11" => run_day(11, || day11::run()),
        "12" => run_day(12, || day12::run()),
        "13" => run_day(13, || day13::run()),
        "14" => run_day(14, || day14::run()),
        "15" => run_day(15, || day15::run()),
        "16" => run_day(16, || day16::run()),
        "17" => run_day(17, || day17::run()),
        "18" => run_day(18, || day18::run()),
        _ => log::error!("Unimplemented day {}", day),
    }
}

fn run_day(day_num: u8, day_func: impl Fn() -> Result<DayResult, Box<dyn Error>>) {
    log::info!("Starting day {}", day_num);
    let now = Instant::now();
    match day_func() {
        Ok(r) => log::info!("Day {} result:\n{}", day_num, r),
        Err(e) => log::error!("{}", e),
    }
    let elapsed = Instant::now() - now;
    log::info!("Time taken: {} seconds", elapsed.as_secs_f32());
}
