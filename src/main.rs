extern crate clap;

use clap::{App, Arg};
use std::io;

mod day01;
mod day02;
mod day03;

fn main() -> io::Result<()> {
    let args = App::new("Advent of Code 2022")
        .version("0.1.0")
        .author("K. Ebstrup <k.ebstrup@gmail.com>")
        .about("My solution code to the Advent of Code 2022")
        .arg(Arg::new("day").required(true))
        .arg(Arg::new("part").required(true))
        .get_matches();

    let day = match args.value_of("day") {
        Some(s) => match s.parse::<u8>() {
            Ok(d) => d,
            Err(_e) => panic!("Error parsing the day!"),
        },
        None => panic!("No or invalid day input!"),
    };

    let part = match args.value_of("part") {
        Some(s) => match s.parse::<u8>() {
            Ok(p) => p,
            Err(_e) => panic!("Error parsing the part!"),
        },
        None => panic!("No or invalid part input!"),
    };

    // get input file for the day
    let input = aoc2022::input_file(day)?;

    // run the day
    match day {
        1 => day01::solve(input, part)?,
        2 => day02::solve(input, part)?,
        3 => day03::solve(input, part)?,
        _ => unimplemented!(),
    }

    Ok(())
}
