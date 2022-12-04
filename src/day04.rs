use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let sections = parse(input)?;

    let solution = match part {
        1 => part_1(sections),
        2 => part_2(sections),
        _ => unimplemented!(),
    };

    println!("{:?}", solution.expect("No solution"));

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

fn part_1(ranges: Vec<Vec<i32>>) -> Option<i32> {
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

fn part_2(ranges: Vec<Vec<i32>>) -> Option<i32> {
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
