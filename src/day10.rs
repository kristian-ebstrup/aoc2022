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

fn parse(input: impl BufRead) -> io::Result<Vec<Vec<String>>> {
    // parse commands
    Ok(input
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        })
        .collect())
}

fn part_1(commands: &Vec<Vec<String>>) -> Option<i32> {
    let mut cycle: i32 = 0;
    let mut x_register: i32 = 1;
    let mut sum: i32 = 0;
    let mut counter: i32 = 0;

    for command in commands.iter() {
        match command.len() {
            1 => {
                cycle += 1;
                if cycle - (20 + counter * 40) == 0 {
                    sum += cycle * x_register;
                    counter += 1;
                }
            }
            2 => {
                for _ in 0..2 {
                    cycle += 1;
                    if cycle - (20 + counter * 40) == 0 {
                        sum += cycle * x_register;
                        counter += 1;
                    }
                }

                x_register += command[1].parse::<i32>().unwrap();
            }
            _ => panic!("Invalid length!"),
        }
    }

    Some(sum)
}

fn part_2(commands: &Vec<Vec<String>>) -> Option<u32> {
    unimplemented!()
}
