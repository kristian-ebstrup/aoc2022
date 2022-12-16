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
    // parse commands
    Ok(input
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        })
        .collect())
}

fn part_1(commands: &Vec<Vec<String>>) -> Option<i32> {
    let mut cycle: i32 = 0;
    let mut x: i32 = 1;
    let mut sum: i32 = 0;
    let mut counter: i32 = 0;

    for command in commands.iter() {
        match command.len() {
            1 => {
                cycle += 1;
                if cycle - (20 + counter * 40) == 0 {
                    sum += cycle * x;
                    counter += 1;
                }
            }
            2 => {
                for _ in 0..2 {
                    cycle += 1;
                    if cycle - (20 + counter * 40) == 0 {
                        sum += cycle * x;
                        counter += 1;
                    }
                }

                x += command[1].parse::<i32>().unwrap();
            }
            _ => panic!("Invalid length!"),
        }
    }

    Some(sum)
}

struct Screen {
    width: usize,
    pixels: Vec<bool>,
    cycle: i32,
    xpos: i32,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Screen {
            width,
            pixels: vec![false; width * height],
            cycle: 0,
            xpos: 1,
        }
    }

    fn switch(&mut self) {
        self.pixels[(self.cycle - 1) as usize] =
            (0..2).contains(&(self.xpos - (self.cycle - 1) % 40).abs());
    }

    fn draw(&self) -> String {
        let mut s = String::from("\n");
        for (i, pixel) in self.pixels.iter().enumerate() {
            if i % self.width == 0 {
                s.push_str("\n");
            }

            if *pixel {
                s.push('#');
            } else {
                s.push('.');
            }
        }

        s
    }
}

fn part_2(commands: &Vec<Vec<String>>) -> Option<String> {
    let mut crt: Screen = Screen::new(40, 6);

    for command in commands.iter() {
        match command.len() {
            1 => {
                crt.cycle += 1;
                crt.switch();
            }
            2 => {
                crt.cycle += 1;
                crt.switch();
                crt.cycle += 1;
                crt.switch();

                crt.xpos += command[1].parse::<i32>().unwrap();
            }
            _ => panic!(),
        }
    }

    Some(crt.draw())
}
