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

impl Move {
    fn from(s: &(String, String)) -> Move {
        let direction = s.0.as_str();
        let steps = s.1.parse::<i32>().unwrap();

        match direction {
            "U" => Move::Up(steps),
            "D" => Move::Down(steps),
            "L" => Move::Left(steps),
            "R" => Move::Right(steps),
            _ => panic!("Invalid movement!"),
        }
    }
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
}

struct Rope {
    knots: Vec<Knot>,
    tracked_knot_id: usize,
    tracker: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(n_knots: usize) -> Self {
        let mut rope = Rope {
            knots: vec![Knot::new(); n_knots],
            tracked_knot_id: n_knots - 1,
            tracker: HashSet::new(),
        };

        rope.track();

        rope
    }

    fn move_head(&mut self, motion: Move) -> () {
        match motion {
            Move::Up(x) => {
                for _ in 0..x {
                    self.knots[0].y += 1;
                    self.move_knots();
                }
            }
            Move::Down(x) => {
                for _ in 0..x {
                    self.knots[0].y -= 1;
                    self.move_knots();
                }
            }
            Move::Left(x) => {
                for _ in 0..x {
                    self.knots[0].x -= 1;
                    self.move_knots();
                }
            }
            Move::Right(x) => {
                for _ in 0..x {
                    self.knots[0].x += 1;
                    self.move_knots();
                }
            }
        };

        self.track();
    }

    fn move_knots(&mut self) -> () {
        for i in 1..self.knots.len() {
            let (dx, dy) = get_relative_pos(&self.knots[i - 1], &self.knots[i]);
            if dx.abs() == 2 || dy.abs() == 2 {
                if dy == 0 {
                    self.knots[i].x = self.knots[i].x + dx.signum();
                } else if dx == 0 {
                    self.knots[i].y = self.knots[i].y + dy.signum();
                } else {
                    self.knots[i].x = self.knots[i].x + dx.signum();
                    self.knots[i].y = self.knots[i].y + dy.signum();
                }
            }

            self.track();
        }
    }

    fn track(&mut self) -> () {
        self.tracker
            .insert(self.knots[self.tracked_knot_id].get_pos());
    }
}

fn get_relative_pos(knot_1: &Knot, knot_2: &Knot) -> (i32, i32) {
    (knot_1.x - knot_2.x, knot_1.y - knot_2.y)
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

    movements
        .into_iter()
        .for_each(|x| rope.move_head(Move::from(x)));

    Some(rope.tracker.len())
}

fn part_2(movements: &Vec<(String, String)>) -> Option<usize> {
    let mut rope = Rope::new(10);

    movements
        .into_iter()
        .for_each(|x| rope.move_head(Move::from(x)));

    Some(rope.tracker.len())
}
