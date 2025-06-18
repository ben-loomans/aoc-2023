use std::collections::{BTreeSet, VecDeque};

pub fn part_one(input: &str) -> Result<String, String> {
    let answer: u32 = input.lines()
        .try_fold(0, |acc, line| -> Result<u32, String> {
            let card = Card::try_from(line)?;
            Ok(acc + card.points())
        })?;

    Ok(answer.to_string())
}

struct Card {
    winners: BTreeSet<u32>,
    numbers: BTreeSet<u32>,
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = String::from("Trouble parsing str to card");

        fn parse(input: &str) -> Option<Card> {
            let (_, lists) = input.split_once(':')?;
            let (winners, numbers) = lists.split_once('|')?;
            
            let winners = winners.trim()
                .split_ascii_whitespace()
                .map(|num| num.parse::<u32>())
                .collect::<Result<BTreeSet<u32>, _>>()
                .ok()?;

            let numbers = numbers.trim()
                .split_ascii_whitespace()
                .map(|num| num.parse::<u32>())
                .collect::<Result<BTreeSet<u32>, _>>()
                .ok()?;

            Some(
                Card {
                    winners,
                    numbers,
                }
            )
        }

        parse(value).ok_or(err)    
    }
}

impl Card {
    fn points(&self) -> u32 {
        let count = self.matches();

        if count > 0 {
            1 << (count - 1) // 2^(count - 1), i.e. doubles for each count
        } else {
            0
        }
    }

    fn matches(&self) -> usize {
        BTreeSet::intersection(&self.numbers, &self.winners)
            .count()
    }
}

pub fn part_two(input: &str) -> Result<String, String> {
    let mut cards = VecDeque::new();

    let answer = input.lines()
        .try_fold(0, |acc, line| -> Result<usize, String> {
            let card = Card::try_from(line)?;
            let points = card.matches() as usize;
            let mult = cards.pop_front().unwrap_or(0) + 1;

            for i in 0..points {
                if let Some(n) = cards.get_mut(i) {
                    *n += mult;
                } else {
                    cards.push_back(mult);
                }
            }
            // println!("points: {points}, mult: {mult}");
            Ok(acc + mult)
        })?;

    Ok(answer.to_string())
}

const _EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

const _ANSWER: &str = "13";
const _ANSWER_2: &str = "30";

#[test]
fn test_part_one() {
    assert_eq!(_ANSWER, &part_one(_EXAMPLE).unwrap());
}

#[test]
fn test_part_two() {
    assert_eq!(_ANSWER_2, &part_two(_EXAMPLE).unwrap());
}