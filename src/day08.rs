use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let parsed_input = parse(input)?;

    let now = std::time::Instant::now();
    let solution = match part {
        0 => (part_1(&parsed_input), part_2(&parsed_input)),
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

fn parse(input: impl BufRead) -> io::Result<Vec<Vec<u8>>> {
    // collect grid
    Ok(input
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|x| x.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect())
}

fn part_1(terminal_output: &Vec<String>) -> Option<u64> {
    unimplemented!()
}

fn part_2(terminal_output: &Vec<String>) -> Option<u64> {
    unimplemented!()
}
