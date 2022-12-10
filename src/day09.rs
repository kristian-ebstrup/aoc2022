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

#[derive(Clone)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new() -> Self {
        Knot { x: 0, y: 0 }
    }

    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn set_pos(&mut self, pos: (i32, i32)) -> (i32, i32) {
        let prev_pos: (i32, i32) = self.get_pos();
        (self.x, self.y) = pos;

        prev_pos
    }
}

struct Rope {
    knots: Vec<Knot>,
    tracked_knot_id: usize,
    tracker: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(n_tails: usize) -> Self {
        let mut rope = Rope {
            knots: vec![Knot::new(); n_tails],
            tracked_knot_id: n_tails - 1,
            tracker: HashSet::new(),
        };

        rope.track();

        rope
    }

    fn move_head(&mut self, motion: Move) -> () {
        match motion {
            Move::Up(x) => {
                for _ in 0..x {
                    let prev_pos = self.knots[0].get_pos();
                    self.knots[0].set_pos((prev_pos.0, prev_pos.1 + 1));
                    self.move_knots(prev_pos);
                }
            }
            Move::Down(x) => {
                for _ in 0..x {
                    let prev_pos = self.knots[0].get_pos();
                    self.knots[0].set_pos((prev_pos.0, prev_pos.1 - 1));
                    self.move_knots(prev_pos);
                }
            }
            Move::Left(x) => {
                for _ in 0..x {
                    let prev_pos = self.knots[0].get_pos();
                    self.knots[0].set_pos((prev_pos.0 - 1, prev_pos.1));
                    self.move_knots(prev_pos);
                }
            }
            Move::Right(x) => {
                for _ in 0..x {
                    let prev_pos = self.knots[0].get_pos();
                    self.knots[0].set_pos((prev_pos.0 + 1, prev_pos.1));
                    self.move_knots(prev_pos);
                }
            }
        };
    }

    fn move_knots(&mut self, prev_pos: (i32, i32)) -> () {
        let mut prev_pos = prev_pos;
        for i in 1..self.knots.len() {
            match is_adjacent(self.knots[i - 1].get_pos(), self.knots[i].get_pos()) {
                true => (),
                false => {
                    prev_pos = self.knots[i].set_pos(prev_pos);
                }
            }
        }
        self.track();
    }

    fn track(&mut self) -> () {
        self.tracker
            .insert(self.knots[self.tracked_knot_id].get_pos());
    }
}

fn is_adjacent(pos1: (i32, i32), pos2: (i32, i32)) -> bool {
    (pos1.0 - pos2.0).abs() <= 1 && (pos1.1 - pos2.1).abs() <= 1
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
    let mut rope = Rope::new(2);

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

        rope.move_head(motion);
    }

    Some(rope.tracker.len())
}

fn part_2(grid: &Vec<(String, String)>) -> Option<u32> {
    unimplemented!();
}
