# Advent of Code 2022
My solutions to Advent of Code 2022, solved using Rust. The handling of the input files is (somewhat) shamelessly copied from [Basile Henry's Advent of Code 2020](https://github.com/basile-henry/aoc2020/), while the CLI is built using [clap](https://github.com/clap-rs/clap).

# To Run
To run, either run the binary or use `cargo run` with the desired day and part to compute:

    cargo run <day> <part>

or

    aoc2022 <day> <part>

depending on whether the binary is built or not. For example, to run day 1, part 1, simply type:

    cargo run 1 1

or

    aoc2022 1 1

The binary is, of course, cleaner to look at as it does not contain compiler output.
