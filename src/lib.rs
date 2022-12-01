/* https://github.com/basile-henry/aoc2020/blob/main/src/lib.rs */
use std::fs::File;
use std::io;
use std::io::BufReader;

pub fn input_file(day: u8) -> io::Result<BufReader<File>> {
    let input_path = format!("inputs/day_{:0>2}.txt", day);

    Ok(BufReader::new(File::open(input_path)?))
}
