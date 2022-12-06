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

fn part_1(buffer: &String) -> Option<i32> {
    let slice: Vec<char> = buffer.clone().chars().collect_vec();

    let mut counter: i32 = 3;
    for window in slice.windows(4) {
        counter += 1;

        let set: HashSet<&char> = HashSet::from_iter(window.iter());

        if set.len() == 4 {
            return Some(counter);
        }
    }

    None
}

fn part_2(buffer: &String) -> Option<i32> {
    let slice: Vec<char> = buffer.clone().chars().collect_vec();

    let mut counter: i32 = 13;
    for window in slice.windows(14) {
        counter += 1;

        let set: HashSet<&char> = HashSet::from_iter(window.iter());

        if set.len() == 14 {
            return Some(counter);
        }
    }

    None
}
