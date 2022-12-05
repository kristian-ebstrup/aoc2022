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
    /* each line consists of a string of two chars separated by a whitespace
     * and each line will simply be read into a vector */
    let mut strategy: Vec<String> = Vec::new();

    for line in input.lines() {
        let s: String = line?;

        strategy.push(s)
    }
    Ok(strategy)
}

fn part_1(strategy: &Vec<String>) -> Option<i32> {
    /* Find the score of the employed strategy
     * (A, B, C) = (X, Y, Z) -> Rock, Paper, Scissors
     * such that
     * X beats C, but loses to B
     * Y beats A, but loses to C
     * Z beats B, but loses to A
     * The points are as follows:
     * (1, 2, 3) for choosing (X, Y, Z)
     * 6 for winning
     * 3 for a tie
     * 0 for losing */
    let mut score: i32 = 0;

    for game in strategy.into_iter() {
        // naive implementation
        let mut plays = game.split_whitespace();
        let opposing_play = plays.next()?;
        let own_play = plays.next()?;

        match own_play {
            "X" => {
                score += 1;
                match opposing_play {
                    "A" => score += 3,
                    "C" => score += 6,
                    _ => (),
                }
            }
            "Y" => {
                score += 2;
                match opposing_play {
                    "A" => score += 6,
                    "B" => score += 3,
                    _ => (),
                }
            }
            "Z" => {
                score += 3;
                match opposing_play {
                    "B" => score += 6,
                    "C" => score += 3,
                    _ => (),
                }
            }
            &_ => (),
        }
    }

    Some(score)
}

fn part_2(strategy: &Vec<String>) -> Option<i32> {
    /* Similar to part 1, but now
     * (X, Y, Z) -> Lose, Draw, Win
     * and the shape and points need to be tallied based on that. */

    let mut score: i32 = 0;

    for game in strategy.into_iter() {
        // naive implementation
        let mut plays = game.split_whitespace();
        let opposing_play = plays.next()?;
        let own_play = plays.next()?;

        match own_play {
            // lose
            "X" => match opposing_play {
                "A" => score += 3,
                "B" => score += 1,
                "C" => score += 2,
                _ => (),
            },
            // draw
            "Y" => {
                score += 3;
                match opposing_play {
                    "A" => score += 1,
                    "B" => score += 2,
                    "C" => score += 3,
                    _ => (),
                }
            }
            // win
            "Z" => {
                score += 6;
                match opposing_play {
                    "A" => score += 2,
                    "B" => score += 3,
                    "C" => score += 1,
                    _ => (),
                }
            }
            &_ => (),
        }
    }

    Some(score)
}
