extern crate clap;

use clap::{App, Arg};
use std::io;

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
mod day11;
mod day12;

fn main() -> io::Result<()> {
    let args = App::new("Advent of Code 2022")
        .version("0.1.0")
        .author("K. Ebstrup <k.ebstrup@gmail.com>")
        .about("My solution code to the Advent of Code 2022")
        .arg(Arg::new("day").required(false))
        .arg(Arg::new("part").required(false))
        .get_matches();

    let day = match args.value_of("day") {
        Some(s) => match s.parse::<u8>() {
            Ok(d) => d,
            Err(_e) => panic!("Error parsing the day!"),
        },
        None => 0,
    };

    let part = match args.value_of("part") {
        Some(s) => match s.parse::<u8>() {
            Ok(p) => p,
            Err(_e) => panic!("Error parsing the part!"),
        },
        None => 0,
    };

    // run the day
    let mut days_to_run: Vec<u8> = match day {
        0 => (1..9).map(|i| i).collect::<Vec<u8>>(),
        _ => vec![day],
    };

    while !days_to_run.is_empty() {
        let day_to_run = days_to_run.pop().unwrap();

        // get input file for the day
        let input = aoc2022::input_file(day_to_run)?;

        println!("# ---- DAY {:0>2} ---- #", day_to_run);
        match day_to_run {
            1 => day01::solve(input, part)?,
            2 => day02::solve(input, part)?,
            3 => day03::solve(input, part)?,
            4 => day04::solve(input, part)?,
            5 => day05::solve(input, part)?,
            6 => day06::solve(input, part)?,
            7 => day07::solve(input, part)?,
            8 => day08::solve(input, part)?,
            9 => day09::solve(input, part)?,
            10 => day10::solve(input, part)?,
            11 => day11::solve(input, part)?,
            12 => day12::solve(input, part)?,
            _ => unimplemented!(),
        }
        println!("");
    }

    Ok(())
}
