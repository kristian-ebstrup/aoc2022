use priority_queue::PriorityQueue;
use std::cmp::Reverse;
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

fn parse(input: impl BufRead) -> io::Result<Vec<Vec<char>>> {
    Ok(input
        .lines()
        .map(|x| x.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>())
}

fn find_char(map: &Vec<Vec<char>>, target: char) -> Option<(usize, usize)> {
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            if map[j][i] == target {
                return Some((i, j));
            }
        }
    }

    None
}

#[derive(Debug, Eq, Hash, Clone)]
struct Node {
    x: usize,
    y: usize,
}

impl Node {
    fn new(pos: (usize, usize)) -> Node {
        Node { x: pos.0, y: pos.1 }
    }

    fn manhattan(&self, other: &Node) -> Option<u32> {
        Some((self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn part_1(map: &Vec<Vec<char>>) -> Option<u32> {
    let mut map = map.clone();

    // get start and goal coordinates
    let start: Node = Node::new(find_char(&map, 'S').unwrap());
    let end: Node = Node::new(find_char(&map, 'E').unwrap());

    // replace "S" and "E" with their altitude values to simplify the A*-algorithm
    map[start.y][start.x] = 'a';
    map[end.y][end.x] = 'z';

    // initialize the open set and push the coordinates to the starting point ('S')
    // note that the value in the PriorityQueue is the f-value
    let mut open_set: PriorityQueue<Node, Reverse<u32>> = PriorityQueue::new();
    open_set.push(start.clone(), Reverse(start.manhattan(&end).unwrap()));

    // initialize the preceeding set, which is initially empty
    let mut closed_set: PriorityQueue<Node, Reverse<u32>> = PriorityQueue::new();

    // A-star
    while !open_set.is_empty() {
        let (node, f) = open_set.pop().unwrap();

        // compute new g-value from (f - h)
        let new_g = f.0 - node.manhattan(&end).unwrap() + 1;

        if node == end {
            return Some(f.0);
        }

        let (x, y) = (node.x, node.y);

        // check left
        if x > 0 {
            if (map[y][x] as i8 - map[y][x - 1] as i8) > -2 {
                // create new node
                let new_node = Node::new((x - 1, y));

                // compute h and f values
                let new_h: u32 = new_node.manhattan(&end).unwrap();
                let new_f: u32 = new_g + new_h;

                // check if node exists anywhere, and handle it accordingly
                match closed_set.get_priority(&new_node) {
                    Some(p) => {
                        if p.0 < new_f {
                            open_set.push_increase(new_node, Reverse(new_f))
                        } else {
                            None
                        }
                    }
                    None => open_set.push_increase(new_node, Reverse(new_f)),
                };
            }
        }
        // check right
        if x < map[0].len() - 1 {
            if (map[y][x] as i8 - map[y][x + 1] as i8) > -2 {
                // create new node
                let new_node = Node::new((x + 1, y));

                // compute h and f values
                let new_h: u32 = new_node.manhattan(&end).unwrap();
                let new_f: u32 = new_g + new_h;

                // check if node exists anywhere, and handle it accordingly
                match closed_set.get_priority(&new_node) {
                    Some(p) => {
                        if p.0 < new_f {
                            open_set.push_increase(new_node, Reverse(new_f))
                        } else {
                            None
                        }
                    }
                    None => open_set.push_increase(new_node, Reverse(new_f)),
                };
            }
        }
        // check down
        if y < map.len() - 1 {
            if (map[y][x] as i8 - map[y + 1][x] as i8) > -2 {
                // create new node
                let new_node = Node::new((x, y + 1));

                // compute h and f values
                let new_h: u32 = new_node.manhattan(&end).unwrap();
                let new_f: u32 = new_g + new_h;

                // check if node exists anywhere, and handle it accordingly
                match closed_set.get_priority(&new_node) {
                    Some(p) => {
                        if p.0 < new_f {
                            open_set.push_increase(new_node, Reverse(new_f))
                        } else {
                            None
                        }
                    }
                    None => open_set.push_increase(new_node, Reverse(new_f)),
                };
            }
        }
        // check up
        if y > 0 {
            if (map[y][x] as i8 - map[y - 1][x] as i8) > -2 {
                // create new node
                let new_node = Node::new((x, y - 1));

                // compute h and f values
                let new_h: u32 = new_node.manhattan(&end).unwrap();
                let new_f: u32 = new_g + new_h;

                // check if node exists anywhere, and handle it accordingly
                match closed_set.get_priority(&new_node) {
                    Some(p) => {
                        if p.0 < new_f {
                            open_set.push_increase(new_node, Reverse(new_f))
                        } else {
                            None
                        }
                    }
                    None => open_set.push_increase(new_node, Reverse(new_f)),
                };
            }
        }

        closed_set.push(node, f);
    }

    panic!("No path found!")
}

fn part_2(map: &Vec<Vec<char>>) -> Option<u32> {
    let mut map = map.clone();

    // get start and goal coordinates
    let start: Node = Node::new(find_char(&map, 'S').unwrap());
    let end: Node = Node::new(find_char(&map, 'E').unwrap());

    // replace "S" and "E" with their altitude values to simplify the A*-algorithm
    map[start.y][start.x] = 'a';
    map[end.y][end.x] = 'z';

    // PART 2 ONLY
    // Find all 'a' to iterate through all possible starting positions
    let mut vec_start: Vec<Node> = Vec::new();
    while find_char(&map, 'a').is_some() {
        let (x, y) = find_char(&map, 'a').unwrap();
        vec_start.push(Node::new((x, y)));
        map[y][x] = '.';
    }

    // iterate through vec_start to replace the dots with 'a' again
    for node in vec_start.iter() {
        map[node.y][node.x] = 'a';
    }

    // iterate through vec_start and find the route values
    let mut vec_values: Vec<u32> = Vec::new();

    for start in vec_start.into_iter() {
        println!("RUNNING START: {:?}", start);

        // initialize the open set and push the coordinates to the starting point ('S')
        // note that the value in the PriorityQueue is the f-value
        let mut open_set: PriorityQueue<Node, Reverse<u32>> = PriorityQueue::new();
        open_set.push(start.clone(), Reverse(start.manhattan(&end).unwrap()));

        // initialize the preceeding set, which is initially empty
        let mut closed_set: PriorityQueue<Node, Reverse<u32>> = PriorityQueue::new();

        // A-star
        while !open_set.is_empty() {
            let (node, f) = open_set.pop().unwrap();

            // compute new g-value from (f - h)
            let new_g = f.0 - node.manhattan(&end).unwrap() + 1;

            if !vec_values.is_empty() {
                if &f.0 > vec_values.iter().max().unwrap() {
                    break;
                }
            }

            if node == end {
                vec_values.push(f.0);
                println!("BREAK!");
                break;
            }

            let (x, y) = (node.x, node.y);

            // check left
            if x > 0 {
                if (map[y][x] as i8 - map[y][x - 1] as i8) > -2 {
                    // create new node
                    let new_node = Node::new((x - 1, y));

                    // compute h and f values
                    let new_h: u32 = new_node.manhattan(&end).unwrap();
                    let new_f: u32 = new_g + new_h;

                    // check if node exists anywhere, and handle it accordingly
                    match closed_set.get_priority(&new_node) {
                        Some(p) => {
                            if p.0 < new_f {
                                open_set.push_increase(new_node, Reverse(new_f))
                            } else {
                                None
                            }
                        }
                        None => open_set.push_increase(new_node, Reverse(new_f)),
                    };
                }
            }
            // check right
            if x < map[0].len() - 1 {
                if (map[y][x] as i8 - map[y][x + 1] as i8) > -2 {
                    // create new node
                    let new_node = Node::new((x + 1, y));

                    // compute h and f values
                    let new_h: u32 = new_node.manhattan(&end).unwrap();
                    let new_f: u32 = new_g + new_h;

                    // check if node exists anywhere, and handle it accordingly
                    match closed_set.get_priority(&new_node) {
                        Some(p) => {
                            if p.0 < new_f {
                                open_set.push_increase(new_node, Reverse(new_f))
                            } else {
                                None
                            }
                        }
                        None => open_set.push_increase(new_node, Reverse(new_f)),
                    };
                }
            }
            // check down
            if y < map.len() - 1 {
                if (map[y][x] as i8 - map[y + 1][x] as i8) > -2 {
                    // create new node
                    let new_node = Node::new((x, y + 1));

                    // compute h and f values
                    let new_h: u32 = new_node.manhattan(&end).unwrap();
                    let new_f: u32 = new_g + new_h;

                    // check if node exists anywhere, and handle it accordingly
                    match closed_set.get_priority(&new_node) {
                        Some(p) => {
                            if p.0 < new_f {
                                open_set.push_increase(new_node, Reverse(new_f))
                            } else {
                                None
                            }
                        }
                        None => open_set.push_increase(new_node, Reverse(new_f)),
                    };
                }
            }
            // check up
            if y > 0 {
                if (map[y][x] as i8 - map[y - 1][x] as i8) > -2 {
                    // create new node
                    let new_node = Node::new((x, y - 1));

                    // compute h and f values
                    let new_h: u32 = new_node.manhattan(&end).unwrap();
                    let new_f: u32 = new_g + new_h;

                    // check if node exists anywhere, and handle it accordingly
                    match closed_set.get_priority(&new_node) {
                        Some(p) => {
                            if p.0 < new_f {
                                open_set.push_increase(new_node, Reverse(new_f))
                            } else {
                                None
                            }
                        }
                        None => open_set.push_increase(new_node, Reverse(new_f)),
                    };
                }
            }

            closed_set.push(node, f);
        }
    }

    vec_values.into_iter().min()
}
