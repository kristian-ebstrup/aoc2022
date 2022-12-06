use itertools::Itertools;
use std::collections::HashSet;
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

fn parse(input: impl BufRead) -> io::Result<String> {
    // one single line of chars
    Ok(input.lines().next().unwrap()?)
}

fn part_1(buffer: &String) -> Option<usize> {
    let slice: Vec<char> = buffer.clone().chars().collect_vec();

    let n: usize = 4;
    Some(
        slice
            .windows(n)
            .map(|x| HashSet::<char>::from_iter(x.to_owned()).len() == n)
            .position(|x| x)
            .unwrap()
            + n,
    )
}

fn part_2(buffer: &String) -> Option<usize> {
    let slice: Vec<char> = buffer.clone().chars().collect_vec();

    let n: usize = 14;
    Some(
        slice
            .windows(n)
            .map(|x| HashSet::<char>::from_iter(x.to_owned()).len() == n)
            .position(|x| x)
            .unwrap()
            + n,
    )
}
