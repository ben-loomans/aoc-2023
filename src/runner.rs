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
        (5, 1) => &day_5::part_one,
        (5, 2) => &day_5::part_two,
        (6, 1) => &day_6::part_one,
        (6, 2) => &day_6::part_two,
        (7, 1) => &day_7::part_one,
        (7, 2) => &day_7::part_two,
        (8, 1) => &day_8::part_one,
        (8, 2) => &day_8::part_two,
        //(9, 1) => &day_9::part_one,
        //(9, 2) => &day_9::part_two,
        //(10, 1) => &day_10::part_one,
        //(10, 2) => &day_10::part_two,
        //(11, 1) => &day_11::part_one,
        //(11, 2) => &day_11::part_two,
        //(12, 1) => &day_12::part_one,
        //(12, 2) => &day_12::part_two,
        //(13, 1) => &day_13::part_one,
        //(13, 2) => &day_13::part_two,
        //(13, 1) => &day_13::part_one,
        //(13, 2) => &day_13::part_two,
        //(14, 1) => &day_14::part_one,
        //(14, 2) => &day_14::part_two,
        //(15, 1) => &day_15::part_one,
        //(15, 2) => &day_15::part_two,
        //(16, 1) => &day_16::part_one,
        //(16, 2) => &day_16::part_two,
        //(17, 1) => &day_17::part_one,
        //(17, 2) => &day_17::part_two,
        //(18, 1) => &day_18::part_one,
        //(18, 2) => &day_18::part_two,
        //(19, 1) => &day_19::part_one,
        //(19, 2) => &day_19::part_two,
        //(20, 1) => &day_20::part_one,
        //(20, 2) => &day_20::part_two,
        //(21, 1) => &day_21::part_one,
        //(21, 2) => &day_21::part_two,
        //(22, 1) => &day_22::part_one,
        //(22, 2) => &day_22::part_two,
        //(23, 1) => &day_23::part_one,
        //(23, 2) => &day_23::part_two,
        //(24, 1) => &day_24::part_one,
        //(24, 2) => &day_24::part_two,
        //(25, 1) => &day_25::part_one,
        //(25, 2) => &day_25::part_two,
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