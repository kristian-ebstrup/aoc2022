use itertools::Itertools;
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

fn parse(input: impl BufRead) -> io::Result<Vec<(Value, Value)>> {
    Ok(input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut xs| {
            (
                Value::parse(&(&mut xs).next().unwrap().unwrap()).0,
                Value::parse(&(&mut xs).next().unwrap().unwrap()).0,
            )
        })
        .collect::<Vec<(Value, Value)>>())
}

#[derive(Debug)]
enum Value {
    Single(u8),
    List(Vec<Value>),
}

impl Value {
    fn parse(input: &str) -> (Self, &str) {
        match input.strip_prefix('[') {
            Some(mut list) => {
                let mut out = Vec::new();

                loop {
                    if let Some(rest) = list.strip_prefix(',') {
                        list = rest;
                    }

                    if let Some(rest) = list.strip_prefix(']') {
                        return (Value::List(out), rest);
                    }

                    let (v, rest) = Value::parse(&mut list);
                    out.push(v);
                    list = rest;
                }
            }
            None => {
                dbg!(input);
                let ix = input.find(|x: char| !x.is_ascii_digit()).unwrap();
                let (num, rest) = input.split_at(ix);
                (Value::Single(num.parse().unwrap()), rest)
            }
        }
    }
}

fn part_1(map: &[(Value, Value)]) -> Option<u32> {
    dbg!(map);
    Some(0)
}

fn part_2(map: &[(Value, Value)]) -> Option<u32> {
    Some(0)
}
