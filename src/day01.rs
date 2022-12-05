use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Elf {
    pub inventory: Vec<i32>,
}

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let elves = parse(input)?;

    let now = std::time::Instant::now();
    let solution = match part {
        0 => (part_1(&elves), part_2(&elves)),
        1 => (part_1(&elves), None),
        2 => (None, part_2(&elves)),
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

fn parse(input: impl BufRead) -> io::Result<Vec<Elf>> {
    let mut elves: Vec<Elf> = Vec::new();
    elves.push(Elf {
        inventory: Vec::new(),
    });

    let mut i: usize = 0;

    for line in input.lines() {
        let s: String = line?;

        if s.is_empty() {
            elves.push(Elf {
                inventory: Vec::new(),
            });

            i = i + 1;
        } else {
            let calories: i32 = match s.parse::<i32>() {
                Ok(cal) => cal,
                Err(_err) => panic!("Error parsing calories!"),
            };
            elves[i].inventory.push(calories);
        }
    }

    Ok(elves)
}

fn part_1(elves: &Vec<Elf>) -> Option<i32> {
    /* Find the most calories carried! */
    let mut most_calories: i32 = 0;

    for elf in elves.iter() {
        let sum_of_calories: i32 = elf.inventory.iter().sum();

        if sum_of_calories >= most_calories {
            most_calories = sum_of_calories;
        }
    }

    Some(most_calories)
}

fn part_2(elves: &Vec<Elf>) -> Option<i32> {
    /* Find the top three elves carrying the most calories! */
    let mut most_calories: Vec<i32> = vec![0, 0, 0];

    for elf in elves.iter() {
        let sum_of_calories: i32 = elf.inventory.iter().sum();

        if most_calories.iter().any(|&x| x <= sum_of_calories) {
            let min_val = most_calories.iter().min()?;
            let min_idx = most_calories.iter().position(|x| x == min_val)?;

            most_calories.swap_remove(min_idx);
            most_calories.push(sum_of_calories);
        }
    }

    Some(most_calories.into_iter().sum())
}
