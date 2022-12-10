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

enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

struct Rope {
    head_pos: (i32, i32),
    tail_pos: (i32, i32),
    tail_set: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Self {
        let mut rope = Rope {
            head_pos: (0, 0),
            tail_pos: (0, 0),
            tail_set: HashSet::new(),
        };

        rope.tail_set.insert((0, 0));

        rope
    }

    fn motion(&mut self, motion: Move) -> () {
        match motion {
            Move::Up(x) => {
                for _ in 0..x {
                    self.head_pos.1 += 1;
                    self.move_tail((self.head_pos.0, self.head_pos.1 - 1));
                    println!("H: {:?}, T: {:?}", self.head_pos, self.tail_pos);
                }
            }
            Move::Down(x) => {
                for _ in 0..x {
                    self.head_pos.1 -= 1;
                    self.move_tail((self.head_pos.0, self.head_pos.1 + 1));
                    println!("H: {:?}, T: {:?}", self.head_pos, self.tail_pos);
                }
            }
            Move::Left(x) => {
                for _ in 0..x {
                    self.head_pos.0 -= 1;
                    self.move_tail((self.head_pos.0 + 1, self.head_pos.1));
                    println!("H: {:?}, T: {:?}", self.head_pos, self.tail_pos);
                }
            }
            Move::Right(x) => {
                for _ in 0..x {
                    self.head_pos.0 += 1;
                    self.move_tail((self.head_pos.0 - 1, self.head_pos.1));
                    println!("H: {:?}, T: {:?}", self.head_pos, self.tail_pos);
                }
            }
        };
    }

    fn move_tail(&mut self, pos: (i32, i32)) -> () {
        match self.is_adjacent() {
            true => (),
            false => {
                self.tail_pos = pos;
                self.tail_set.insert(self.tail_pos);
            }
        }
    }

    fn is_adjacent(&self) -> bool {
        (self.head_pos.0 - self.tail_pos.0).abs() <= 1
            && (self.head_pos.1 - self.tail_pos.1).abs() <= 1
    }
}

fn parse(input: impl BufRead) -> io::Result<Vec<(String, String)>> {
    Ok(input
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<(String, String)>>())
}

fn part_1(movements: &Vec<(String, String)>) -> Option<usize> {
    let mut rope = Rope::new();

    for (direction, steps) in movements.into_iter() {
        let steps_i32 = steps.parse::<i32>().unwrap();
        let direction_str = direction.as_str();

        let motion: Move = match direction_str {
            "U" => Move::Up(steps_i32),
            "D" => Move::Down(steps_i32),
            "L" => Move::Left(steps_i32),
            "R" => Move::Right(steps_i32),
            _ => panic!("Invalid movement!"),
        };

        rope.motion(motion);
    }

    Some(rope.tail_set.len())
}

fn part_2(grid: &Vec<(String, String)>) -> Option<u32> {
    unimplemented!();
}
