use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::Parser;
use nom::{sequence::delimited, IResult};
use nom::character::complete::usize;

pub fn part_one(input: &str) -> Result<String, String> {
    process_input(input).map(|answer| answer.to_string())
}

struct Game {
    id: usize,
    draws: Vec<Draw>
}

#[derive(Default)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

enum Colour {
    Red(usize),
    Green(usize),
    Blue(usize),
}

const BAG: Draw = Draw {
    red: 12,
    green: 13,
    blue: 14,
};

impl Game {
    fn min_draw(&self) -> Draw {
        self.draws.iter()
            .fold(Draw::default(), |acc, draw| {
                Draw {
                    red: acc.red.max(draw.red),
                    green: acc.green.max(draw.green),
                    blue: acc.blue.max(draw.blue),
                }
            })
    }
}

impl Draw {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn process_input(input: &str) -> Result<usize, String> {
    input.lines().try_fold(0, |acc, line| {
        let game = parse_game(line)?;
        if is_game_possible(&game, &BAG) {
            Ok(acc + game.id)
        } else {
            Ok(acc)
        }
    })
}

fn parse_game(input: &str) -> Result<Game, String> {
    let (_, (id, draws)) = all_consuming((
        delimited(
            tag("Game "), 
            usize, 
            tag(": ")
        ),
        separated_list1(
            tag("; "),
            parse_draw
        )
    )).parse(input)
        .map_err(|_| "issue parsing game".to_string())?;

    Ok(Game {
        id,
        draws,
    })
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    // e.g. input = "8 green, 6 blue, 20 red"
    let mut draw = Draw::default();

    let (rest, colours) = separated_list1(
        tag(", "), 
        parse_colour
    ).parse(input)?;
    
    for colour in colours {
        match colour {
            Colour::Red(num) => draw.red += num,
            Colour::Green(num) => draw.green += num,
            Colour::Blue(num) => draw.blue += num,
        }
    }

    return Ok((rest, draw))
}

fn parse_colour(input: &str) -> IResult<&str, Colour> {
    alt((
        map(
            terminated(
                usize, 
                tag(" red"),
            ),
            |num| Colour::Red(num)
        ),
        map(
            terminated(
                usize, 
                tag(" green"),
            ),
            |num| Colour::Green(num)
        ),
        map(
            terminated(
                usize, 
                tag(" blue"),
            ),
            |num| Colour::Blue(num)
        )
    )).parse(input)
}

fn is_game_possible(game: &Game, bag: &Draw) -> bool {
    for draw in &game.draws {
        if  draw.red > bag.red || 
            draw.green > bag.green ||
            draw.blue > bag.blue 
        {
            return false;
        }
    }

    return true;
}

pub fn part_two(input: &str) -> Result<String, String> {
    parse_two(input).map(|answer| answer.to_string())
}

fn parse_two(input: &str) -> Result<usize, String> {
    input.lines().try_fold(0, |acc, line| {
        let game = parse_game(line)?;
        let power = game.min_draw().power();

        Ok(acc + power)
    })
}

const _EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

const _ANSWER: &str = "8";

#[test]
fn test_part_one() {
    // assert_eq!(_ANSWER, &part_one(_EXAMPLE).unwrap());
}

#[test]
fn test_part_two() {
    // assert_eq!(_ANSWER, &part_two(_EXAMPLE).unwrap());
}