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

fn parse(input: impl BufRead) -> io::Result<Vec<String>> {
    /* each line corresponds to a rucksack, where each character
     * is a case-sensitive item. */
    let rucksacks: Vec<String> = input
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    Ok(rucksacks)
}

fn part_1(rucksacks: &Vec<String>) -> Option<i32> {
    /* Each rucksack has two compartments of equal size,
     * i.e. the first half of the line is the contents of the
     * first compartment, and the second half represents the
     * second compartment.
     * Compare compartments, find items occuring in both compartments,
     * and compute the priority value */
    let mut priority_sum: i32 = 0;

    for rucksack in rucksacks.into_iter() {
        let (left, right) = rucksack.split_at(rucksack.len() / 2);
        // build HashSets for each compartment
        let set_l: HashSet<char> = HashSet::from_iter(left.chars());
        let set_r: HashSet<char> = HashSet::from_iter(right.chars());

        // check intersection
        let intersection: char = *set_l.intersection(&set_r).next().unwrap();

        // match to whether it is uppercase, and compute priority value from there
        priority_sum += match intersection.is_uppercase() {
            true => intersection as i32 - (65 - 27),
            false => intersection as i32 - 96,
        };
    }

    Some(priority_sum)
}

fn part_2(rucksacks: &Vec<String>) -> Option<i32> {
    /* group the rucksack in sets of three, and find
     * the only item that occurs in all three sets:
     * this is the badge, and defines the priority value */
    let mut priority_sum: i32 = 0;

    let mut sacks = rucksacks.iter().tuples();
    while let Some((one, two, three)) = sacks.next() {
        // define three HashSets for the three sacks
        let sack_1: HashSet<char> = HashSet::from_iter(one.chars());
        let sack_2: HashSet<char> = HashSet::from_iter(two.chars());
        let sack_3: HashSet<char> = HashSet::from_iter(three.chars());

        // compare sack 1 and 2 first
        let intersection_1: HashSet<char> = sack_1.intersection(&sack_2).map(|&x| x).collect();
        let intersection_2: char = *intersection_1.intersection(&sack_3).next().unwrap();

        // match to whether it is uppercase, and compute priority value from there
        priority_sum += match intersection_2.is_uppercase() {
            true => intersection_2 as i32 - (65 - 27),
            false => intersection_2 as i32 - 96,
        };
    }

    Some(priority_sum)
}
