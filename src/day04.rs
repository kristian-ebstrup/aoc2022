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

fn parse(input: impl BufRead) -> io::Result<Vec<Vec<i32>>> {
    let mut sections: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let s = line?;

        // split at the comma and dash, and push to sections vector
        let str_vec: Vec<&str> = s.split(&['-', ','][..]).collect();
        let mut i32_vec: Vec<i32> = Vec::new();
        for c in str_vec.into_iter() {
            i32_vec.push(c.parse::<i32>().unwrap());
        }

        sections.push(i32_vec);
    }
    Ok(sections)
}

fn part_1(ranges: &Vec<Vec<i32>>) -> Option<i32> {
    let mut n_contained_ranges: i32 = 0;

    // for each set of section ranges, check if either range
    // is a subrange of the other (i.e. is fully contained)
    for range in ranges.into_iter() {
        let start_cmp: i32 = range[0] - range[2];
        let end_cmp: i32 = range[3] - range[1];

        if start_cmp.signum() == end_cmp.signum() || start_cmp == 0 || end_cmp == 0 {
            n_contained_ranges += 1;
        }
    }

    Some(n_contained_ranges)
}

fn part_2(ranges: &Vec<Vec<i32>>) -> Option<i32> {
    let mut n_overlapping_ranges: i32 = 0;

    // for each set of section ranges, check if either range
    // overlaps at all.
    for range in ranges.into_iter() {
        if (range[0] >= range[2] && range[0] <= range[3])
            || (range[1] >= range[2] && range[1] <= range[3])
            || (range[2] >= range[0] && range[2] <= range[1])
            || (range[3] >= range[0] && range[3] <= range[1])
        {
            n_overlapping_ranges += 1;
        }
    }

    Some(n_overlapping_ranges)
}
