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

fn parse(input: impl BufRead) -> io::Result<Vec<Vec<String>>> {
    Ok(input
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .skip(2)
                .map(|s| s.split(',').next().unwrap().to_string())
                .collect()
        })
        .filter(|v: &Vec<String>| !v.is_empty())
        .collect())
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: (String, String),
    test: (u32, u32, u32),
}

impl Monkey {
    fn new(
        state: (
            &Vec<String>,
            &Vec<String>,
            &Vec<String>,
            &Vec<String>,
            &Vec<String>,
        ),
    ) -> Self {
        /* The state input is a bit cryptic due to the way the parsing is handled. Effectively,
         * each monkey is through the parsing divided into their respective information, with all
         * lines skipping the first two words as they hold no information for all lines. From here,
         * which word is of interest depends on the line, and thus we end up with a seemingly
         * incomprehensible monkey state input. However, it should be as follows:
         *
         * state.0: A vector of numbers in String format corresponding to the items.
         * state.1: A vector of length 4, where the third and fourth element corresponds to the
         *          operation and operation input.
         * state.2: A vector of length 2, where the last element is the test value (divisible by X)
         * state.3: A vector of length 4, where the last element is the target monkey if test is
         *          true
         * state.4: A vector of length 4, where the last element is the target monkey if test is
         *          false
         *
         * Everything else is unimportant. */

        // read items
        let items = state.0.iter().map(|s| s.parse::<u32>().unwrap()).collect();

        // read operation
        let operation = (state.1[2].clone(), state.1[3].clone());

        // read test
        let test = (
            state.2[1].parse::<u32>().unwrap(),
            state.3[3].parse::<u32>().unwrap(),
            state.4[3].parse::<u32>().unwrap(),
        );

        Monkey {
            items,
            operation,
            test,
        }
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn new() -> Self {
        Monkeys {
            monkeys: Vec::new(),
        }
    }

    fn add(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }
}

fn part_1(input: &Vec<Vec<String>>) -> Option<i32> {
    let mut monkeys: Monkeys = Monkeys::new();

    for (i, state) in input
        .iter()
        .tuples::<(
            &Vec<String>,
            &Vec<String>,
            &Vec<String>,
            &Vec<String>,
            &Vec<String>,
        )>()
        .enumerate()
    {
        monkeys.add(Monkey::new(state));
    }

    println!("{:?}", monkeys);

    Some(0)
}

fn part_2(commands: &Vec<Vec<String>>) -> Option<u32> {
    Some(0)
}
