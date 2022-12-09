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

struct TreeGrid {
    rows: Vec<Vec<u8>>,
    cols: Vec<Vec<u8>>,
}

impl TreeGrid {
    fn new(grid: &Vec<Vec<u8>>) -> Self {
        let mut cols: Vec<Vec<u8>> = vec![Vec::with_capacity(grid[0].len()); grid.len()];
        for r in grid {
            for (j, c) in r.iter().enumerate() {
                cols[j].push(*c);
            }
        }

        TreeGrid {
            rows: grid.clone(),
            cols,
        }
    }

    fn get_width(&self) -> usize {
        self.rows[0].len()
    }

    fn get_height(&self) -> usize {
        self.cols[0].len()
    }
}

fn parse(input: impl BufRead) -> io::Result<Vec<Vec<u8>>> {
    // collect grid
    Ok(input
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|x| x.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect())
}

fn part_1(grid: &Vec<Vec<u8>>) -> Option<u64> {
    let tree_grid = TreeGrid::new(grid);

    let grid_height: usize = tree_grid.get_height();
    let grid_width: usize = tree_grid.get_width();

    let n_edge_trees: usize = 2 * (grid_width + grid_height) - 4;
    let mut visible_trees: u64 = n_edge_trees as u64;

    // iterate through all trees
    for i in 1..(grid_width - 1) {
        for j in 1..(grid_height - 1) {
            let tree_height = &grid[i][j];

            // check horizontal
            let check_left: bool = tree_grid.rows[i]
                .iter()
                .enumerate()
                .filter(|(c, _)| c < &j)
                .all(|(_, v)| v < tree_height);
            let check_right: bool = tree_grid.rows[i]
                .iter()
                .enumerate()
                .filter(|(c, _)| c > &j)
                .all(|(_, v)| v < tree_height);

            // check vertical
            let check_top: bool = tree_grid.cols[j]
                .iter()
                .enumerate()
                .filter(|(r, _)| r < &i)
                .all(|(_, v)| v < tree_height);
            let check_bottom: bool = tree_grid.cols[j]
                .iter()
                .enumerate()
                .filter(|(r, _)| r > &i)
                .all(|(_, v)| v < tree_height);

            if check_left || check_right || check_top || check_bottom {
                visible_trees += 1;
            }
        }
    }

    Some(visible_trees)
}

fn part_2(grid: &Vec<Vec<u8>>) -> Option<u64> {
    unimplemented!()
}
