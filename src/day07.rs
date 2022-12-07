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

fn parse(input: impl BufRead) -> io::Result<Vec<String>> {
    // collect all terminal commands and output into a single vector
    Ok(input
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.contains("dir"))
        .collect())
}

fn get_directory_sizes(terminal_output: &Vec<String>) -> Option<Vec<u64>> {
    let mut directories: Vec<u64> = Vec::new();
    let mut active_directories: Vec<usize> = Vec::new();

    for instr in terminal_output.into_iter() {
        let split: Vec<&str> = instr.split_whitespace().map(|x| x).collect();
        match split[0] {
            // command
            "$" => {
                match split[1] {
                    "cd" => {
                        if split[2] == ".." {
                            let cascade = directories[active_directories.pop().unwrap()].clone();
                            directories[*active_directories.last().unwrap()] += cascade;
                        } else {
                            directories.push(0);
                            active_directories.push(directories.len() - 1);
                        }
                    }
                    _ => (),
                };
            }
            // list output
            _ => {
                directories[*active_directories.last().unwrap()] +=
                    split[0].parse::<u64>().unwrap();
            }
        }
    }

    // purge active_directories to include the remaining sizes
    while !active_directories.is_empty() {
        let cascade = directories[active_directories.pop().unwrap()].clone();
        match active_directories.last() {
            Some(&i) => directories[i] += cascade,
            None => (),
        }
    }

    Some(directories)
}

fn part_1(terminal_output: &Vec<String>) -> Option<u64> {
    let directories: Vec<u64> = get_directory_sizes(terminal_output).unwrap();

    Some(
        directories
            .into_iter()
            .filter(|&x| x <= 100000)
            .sum::<u64>(),
    )
}

fn part_2(terminal_output: &Vec<String>) -> Option<u64> {
    let directories: Vec<u64> = get_directory_sizes(terminal_output).unwrap();

    let available_space: u64 = 70000000;
    let required_space: u64 = 30000000;

    let current_space: u64 = *directories.first().unwrap();
    let unused_space: u64 = available_space - current_space;
    let space_to_be_found: u64 = required_space - unused_space;

    directories
        .into_iter()
        .filter(|&x| x >= space_to_be_found)
        .min()
}
