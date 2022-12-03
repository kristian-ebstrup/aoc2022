use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let rucksacks = parse(input)?;

    let solution = match part {
        1 => part_1(rucksacks),
        2 => part_2(rucksacks),
        _ => unimplemented!(),
    };

    println!("{:?}", solution.expect("No solution"));

    Ok(())
}

fn parse(input: impl BufRead) -> io::Result<Vec<String>> {
    /* each line corresponds to a rucksack, where each character
     * is a case-sensitive item. */
    let mut rucksacks: Vec<String> = Vec::new();

    for line in input.lines() {
        let s = line?;
        rucksacks.push(s);
    }
    Ok(rucksacks)
}

fn part_1(rucksacks: Vec<String>) -> Option<i32> {
    /* Each rucksack has two compartments of equal size,
     * i.e. the first half of the line is the contents of the
     * first compartment, and the second half represents the
     * second compartment.
     * Compare compartments, find items occuring in both compartments,
     * and compute the priority value */
    let mut priority_sum: i32 = 0;

    for rucksack in rucksacks.into_iter() {
        let (left, right) = rucksack.split_at(rucksack.len() / 2);
        // build HashMaps for each compartment
        let mut map_left: HashMap<char, u8> = HashMap::new();
        for c in left.chars() {
            map_left.insert(c, 1);
        }

        let mut map_right: HashMap<char, u8> = HashMap::new();
        for c in right.chars() {
            map_right.insert(c, 1);
        }

        // extract keys existing in both maps
        for key in map_left.keys() {
            if map_right.contains_key(key) {
                let shared_key = key;
                // convert to ASCII value, and correct to get priority value
                if key.is_uppercase() {
                    priority_sum += *shared_key as i32 - (64 - 26);
                } else {
                    priority_sum += *shared_key as i32 - 96;
                }
            }
        }
    }

    Some(priority_sum)
}

fn part_2(rucksacks: Vec<String>) -> Option<i32> {
    /* group the rucksack in sets of three, and find
     * the only item that occurs in all three sets:
     * this is the badge, and defines the priority value */
    let mut priority_sum: i32 = 0;

    // define counter and map outside loop
    let mut counter: u8 = 0;
    let mut reset_counter: u8 = 0;
    let mut map: HashMap<char, u8> = HashMap::new();

    println!("{}", rucksacks.len());
    for rucksack in rucksacks.into_iter() {
        // advance counter, and create a map of banned chars to avoid counting the same
        // char twice in the same rucksack
        counter += 1;
        let mut banned_chars: HashMap<char, u8> = HashMap::new();

        for c in rucksack.chars() {
            if !banned_chars.contains_key(&c) {
                // if key exists, increase by one; else, insert key and value = 1.
                match map.get(&c) {
                    Some(v) => map.insert(c, v + 1),
                    None => map.insert(c, 1),
                };

                banned_chars.insert(c, 1);
            }
        }

        if counter == 3 {
            for (key, val) in map.drain() {
                if val == 3 {
                    let badge = key;
                    // convert to ASCII value, and correct to get priority value
                    if badge.is_uppercase() {
                        priority_sum += badge as i32 - (64 - 26);
                    } else {
                        priority_sum += badge as i32 - 96;
                    }
                }
            }
            counter = 0;
            reset_counter += 1;
        }
    }
    println!("{}", reset_counter);

    Some(priority_sum)
}
