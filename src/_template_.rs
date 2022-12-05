use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let parsed_input = parse(input)?;

    println!("# ---- DAY 05 ---- #");
    let now = std::time::Instant::now();
    let solution = match part {
        0 => (part_1(&parsed_input), part_2(&parsed)),
        1 => (part_1(&parsed_input), None),
        2 => (None, part_2(&parsed_input)),
        _ => unimplemented!(),
    };
    let time = now.elapsed().as_micros();

    match solution.0 {
        Some(x) => println!("Part 1: {}", x),
        None => println!(),
    }
    match solution.1 {
        Some(x) => println!("Part 2: {}", x),
        None => println!(),
    }

    println!("Time elapsed: {} µs", time);

    Ok(())
}

fn parse(input: impl BufRead) -> io::Result<_> {
    unimplemented!();
}

fn part_1(_: &<T>) -> Option<i32> {
    unimplemented!();
}

fn part_2(_: &<T>) -> Option<i32> {
    unimplemented!();
}
