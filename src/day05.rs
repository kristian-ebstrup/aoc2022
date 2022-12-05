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

fn parse(input: impl BufRead) -> io::Result<(Vec<Vec<String>>, Vec<Vec<usize>>)> {
    let mut storage: Vec<Vec<String>> = Vec::new();
    let mut commands: Vec<Vec<usize>> = Vec::new();

    let mut switch: bool = false;
    for line in input.lines() {
        let s = line?;
        if s.is_empty() {
            switch = true;
        } else {
            if switch == false {
                for (i, c) in s.chars().enumerate() {
                    if c.is_alphabetic() {
                        // [1] [5] [9] [13] ...
                        // i.e. the actual index is i / 4
                        let index = i / 4;
                        while storage.len() < index + 1 {
                            storage.push(Vec::new());
                        }
                        storage[index].insert(0, c.to_string());
                    }
                }
            } else {
                let mut numerics: Vec<usize> = Vec::new();
                for c in s
                    .split_whitespace()
                    .filter(|c| c.chars().all(|x| x.is_numeric()))
                {
                    numerics.push(c.to_string().parse::<usize>().unwrap());
                }
                commands.push(numerics);
            }
        }
    }

    Ok((storage, commands))
}

fn part_1(storage_and_commands: &(Vec<Vec<String>>, Vec<Vec<usize>>)) -> Option<String> {
    let (_storage, commands) = storage_and_commands;
    let mut storage: Vec<Vec<String>> = _storage.clone();

    let mut top_crates: String = String::new();

    for command in commands.iter() {
        // get commands parsed into u32
        let mut moves = command[0];
        let idx_from = command[1] - 1;
        let idx_to = command[2] - 1;

        while moves > 0 {
            match storage[idx_from].pop() {
                Some(c) => storage[idx_to].push(c),
                None => (),
            }

            moves -= 1;
        }
    }

    for mut vec in storage.into_iter() {
        match &vec.pop() {
            Some(c) => top_crates.push_str(c),
            None => (),
        }
    }
    Some(top_crates)
}

fn part_2(storage_and_commands: &(Vec<Vec<String>>, Vec<Vec<usize>>)) -> Option<String> {
    let (_storage, commands) = storage_and_commands;
    let mut storage: Vec<Vec<String>> = _storage.clone();

    let mut top_crates: String = String::new();

    for command in commands.iter() {
        // get commands parsed into u32
        let mut moves = command[0];
        let idx_from = command[1] - 1;
        let idx_to = command[2] - 1;

        let mut crane_crates: Vec<String> = Vec::new();
        while moves > 0 {
            match storage[idx_from].pop() {
                Some(c) => crane_crates.push(c),
                None => (),
            }

            moves -= 1;
        }

        crane_crates.reverse();
        for c in crane_crates.into_iter() {
            storage[idx_to].push(c);
        }
    }

    for mut vec in storage.into_iter() {
        match &vec.pop() {
            Some(c) => top_crates.push_str(c),
            None => (),
        }
    }
    Some(top_crates)
}
