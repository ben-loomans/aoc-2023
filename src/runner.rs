use std::fs::read_to_string;

use crate::{cli::Config, solutions::{solved::Solved, *}};

pub fn dispatch(day: u8, part: u8) -> &'static dyn Solved {
    match (day, part) {
        (1, 1) => &day_1::part_one,
        (1, 2) => &day_1::part_two,
        (2, 1) => &day_2::part_one,
        (2, 2) => &day_2::part_two,
        (3, 1) => &day_3::part_one,
        (3, 2) => &day_3::part_two,
        (4, 1) => &day_4::part_one,
        (4, 2) => &day_4::part_two,
        _ => &unimplemented
    }
}

pub fn unimplemented(_input: &str) -> Result<String, String> {
    Err("Unimplemented".to_string())
}

pub fn run(config: Config) {
    match config.runmode {
        crate::cli::Runmode::One { day } => {
            run_day(day);
        },
        crate::cli::Runmode::All => {
            for day in 1..=25 {
                run_day(day);
            }
        },
    }
}

fn run_day(day: u8) {
    println!("Day {day}");

    if let Ok(input) = get_input_from_file(day) {
        println!("Part One");
        dispatch(day, 1).print_timed(&input);
        println!("Part Two");
        dispatch(day, 2).print_timed(&input);
    } else {
        println!("Couldn't open file...");
    }
}

fn get_input_from_file(day: u8) -> Result<String, std::io::Error> {
    let path = format!("inputs/day_{day}.txt");

    read_to_string(path)
}