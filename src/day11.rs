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

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: (String, String),
    test: (u64, usize, usize),
    inspections: usize,
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
        let items = state.0.iter().map(|s| s.parse::<u64>().unwrap()).collect();

        // read operation
        let operation = (state.1[2].clone(), state.1[3].clone());

        // read test
        let test = (
            state.2[1].parse::<u64>().unwrap(),
            state.3[3].parse::<usize>().unwrap(),
            state.4[3].parse::<usize>().unwrap(),
        );

        Monkey {
            items,
            operation,
            test,
            inspections: 0,
        }
    }

    fn inspect_and_throw(&mut self, relief: u64, supermodulo: u64) -> Option<(usize, u64)> {
        self.inspections += 1;
        let mut item = match self.items.pop() {
            Some(x) => x,
            None => return None,
        };

        item = match self.operation.0.as_str() {
            "+" => {
                (item
                    + match self.operation.1.as_str() {
                        "old" => item,
                        _ => self.operation.1.parse::<u64>().unwrap(),
                    })
                    / relief
            }
            "*" => {
                (item
                    * match self.operation.1.as_str() {
                        "old" => item,
                        _ => self.operation.1.parse::<u64>().unwrap(),
                    })
                    / relief
            }
            _ => panic!(
                "Invalid operation ({}, but should be \"+\" or \"*\")",
                self.operation.0
            ),
        };

        item = match supermodulo {
            0 => item,
            _ => item % supermodulo,
        };

        if item % self.test.0 == 0 {
            Some((self.test.1, item))
        } else {
            Some((self.test.2, item))
        }
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    supermodulo: u64,
}

impl Monkeys {
    fn new() -> Self {
        Monkeys {
            monkeys: Vec::new(),
            supermodulo: 0,
        }
    }

    fn add(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    fn compute_supermodulo(&mut self) {
        self.supermodulo = self.monkeys.iter().map(|x| x.test.0).product();
    }

    fn round(&mut self, relief: u64) {
        for i in 0..self.monkeys.len() {
            while !self.monkeys[i].items.is_empty() {
                match self.monkeys[i].inspect_and_throw(relief, self.supermodulo) {
                    Some((target, item)) => self.monkeys[target].items.push(item),
                    None => panic!(),
                }
            }
        }
    }
}

fn part_1(input: &Vec<Vec<String>>) -> Option<usize> {
    let mut monkeys: Monkeys = Monkeys::new();

    for state in input.iter().tuples::<(
        &Vec<String>,
        &Vec<String>,
        &Vec<String>,
        &Vec<String>,
        &Vec<String>,
    )>() {
        monkeys.add(Monkey::new(state));
    }

    for _ in 0..20 {
        monkeys.round(3);
    }

    let mut inspections = monkeys
        .monkeys
        .iter()
        .map(|x| x.inspections)
        .collect::<Vec<usize>>();
    inspections.sort_unstable();

    let monkey_business = inspections[inspections.len() - 1] * inspections[inspections.len() - 2];

    Some(monkey_business)
}

fn part_2(input: &Vec<Vec<String>>) -> Option<usize> {
    let mut monkeys: Monkeys = Monkeys::new();

    for state in input.iter().tuples::<(
        &Vec<String>,
        &Vec<String>,
        &Vec<String>,
        &Vec<String>,
        &Vec<String>,
    )>() {
        monkeys.add(Monkey::new(state));
    }

    monkeys.compute_supermodulo();

    for _ in 0..10000 {
        monkeys.round(1);
    }

    let mut inspections = monkeys
        .monkeys
        .iter()
        .map(|x| x.inspections)
        .collect::<Vec<usize>>();
    inspections.sort_unstable();

    let monkey_business = inspections[inspections.len() - 1] * inspections[inspections.len() - 2];

    Some(monkey_business)
}
