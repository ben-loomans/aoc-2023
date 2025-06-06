use clap::{arg, value_parser, Arg, ArgGroup, Command};

/*
command line interface for advent of code
specify day number as an argument
-t flag to run with test input
-a flag to run all solutions

future:

*/

pub fn get_config() -> Config {
    let matches = Command::new("AoC2024")
    .arg(
        Arg::new("day")
            .help("Which day's problem to solve")
            .index(1)
            .value_parser(value_parser!(u8).range(1..=25))
        )
    .arg(
        arg!(-a --all "Run all solutions")
    )
    .group(
        ArgGroup::new("mode")
            .args(&["day", "all"])
            .required(true)
    )
    .arg(
        arg!(-t --test "Use test input file instead")
    )
    .get_matches();

    let runmode = match matches.get_one::<u8>("day").cloned() {
        Some(day) => {
            Runmode::One { 
            day,
            }
        },
        None => Runmode::All
    };

    Config {
        runmode,
        test: matches.get_flag("test"),
    }
}

pub struct Config {
    pub runmode: Runmode,
    pub test: bool,
}

pub enum Runmode {
    One {
        day: u8,
    },
    All,
}

/*
What is this supposed to do?
Let's break it into parts
1) deciding which solutions need to be run
2) deciding which inputs need to be provided to those functions
3) timing the solutions
4) formatting the outputs of the solutions for the user

questions
1) should the runner keep running if a solution fails?
    yes, but it should display that the solution failed
2) should failed solutions be timed?
    probably not
3) should solutions have a time limit?
    yes that would be good
4) can solutions run in parallel?
    probably not without messing up the timing
5) should printing be progressive or all at once?
    progressive is nicer

*/