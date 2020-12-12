mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod dayerror;
mod interpreter;

#[macro_use]
extern crate lazy_static;

use crate::dayerror::DayError;
use std::{env::args, error::Error, fmt, io, str::FromStr, time::Instant};
use thiserror::Error;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), ApplicationError> {
    let mut args = args();
    let daynum = args.nth(1).expect("Expected a day number argument");
    let day: u8 =
        u8::from_str(&daynum).expect(&format!("Expected day number {} to be a u8", daynum));

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let part1_start = Instant::now();
    let part1 = match day {
        1 => crate::day1::part1()?,
        2 => crate::day2::part1()?,
        3 => crate::day3::part1()?,
        4 => crate::day4::part1()?,
        5 => crate::day5::part1()?,
        6 => crate::day6::part1()?,
        7 => crate::day7::part1()?,
        8 => crate::day8::part1()?,
        9 => crate::day9::part1()?,
        10 => crate::day10::part1()?,
        11 => crate::day11::part1(&mut terminal)?,
        d => return Err(ApplicationError::BadDayError(BadDayError(d))),
    };
    let part1_duration = part1_start.elapsed();

    println!("[Part 1 in {}ms]: {}", part1_duration.as_millis(), part1);

    let part2_start = Instant::now();
    let part2 = match day {
        1 => crate::day1::part2()?,
        2 => crate::day2::part2()?,
        3 => crate::day3::part2()?,
        4 => crate::day4::part2()?,
        5 => crate::day5::part2()?,
        6 => crate::day6::part2()?,
        7 => crate::day7::part2()?,
        8 => crate::day8::part2()?,
        9 => crate::day9::part2()?,
        10 => crate::day10::part2()?,
        d => return Err(ApplicationError::BadDayError(BadDayError(d))),
    };
    let part2_duration = part2_start.elapsed();

    println!("[Part 2 in {}ms]: {}", part2_duration.as_millis(), part2);

    Ok(())
}

#[derive(Debug, Error)]
enum ApplicationError {
    #[error(transparent)]
    DayError(#[from] DayError),
    #[error(transparent)]
    BadDayError(#[from] BadDayError),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

#[derive(Debug, Error)]
#[error("Unknown or invalid day")]
struct BadDayError(u8);
