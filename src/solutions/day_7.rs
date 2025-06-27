use std::{collections::BTreeMap, str::FromStr};

pub fn part_one(input: &str) -> Result<String, String> {
    let hand_bets = parse_input(input)
        .map_err(|_| "trouble parsing input".to_string())?;

    let answer = solve(&hand_bets);

    Ok(answer.to_string())
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Card(u8);

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let val = match value {
            '2'..='9' => value.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => return Err(()),
        };

        Ok(Card(val))
    }
}

impl Card {
    fn jokerify(&mut self) {
        if self.0 == 11 {
            self.0 = 1
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(());
        }

        let cards = s.chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<Card>, ()>>()?;
        
        let cards = cards.try_into().map_err(|_| ())?;

        Ok(
            Hand {
                cards,
            }
        )
    }
}

impl Hand {
    fn find_type(&self) -> HandType {
        let mut cards = self.cards.iter()
            .fold(BTreeMap::new(), |mut cards, card| {
                *cards.entry(card).or_insert(0) += 1;
                cards
            });
        
        let jokers = cards.remove(&Card(1)).unwrap_or(0);

        let mut counts: Vec<i32> = cards.into_values().collect();
        counts.sort_by(|a, b| b.cmp(a));

        // 5 joker edge case handled by unwrap_or
        let first = counts.get(0).unwrap_or(&0) + jokers;
        let second = counts.get(1);

        match (first, second) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, Some(2)) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, Some(2)) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard
        }
    }

    fn jokerify(&mut self) {
        for card in &mut self.cards {
            card.jokerify();
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type Bet = u32;

fn parse_input(input: &str) -> Result<Vec<(Hand, Bet)>, ()> {
    input.lines()
        .map(|line| -> Result<(Hand, u32), ()> {
            let (hand, bet) = line.split_once(' ').ok_or(())?;
            let hand = hand.parse()?;
            let bet = bet.parse().map_err(|_| ())?;

            Ok((hand, bet))
        }).collect()
}

fn solve(input: &[(Hand, Bet)]) -> Bet {
    let mut type_hand_bets: Vec<(HandType, &Hand, &Bet)> = input.iter()
        .map(|(hand, bet)| {
            (hand.find_type(), hand, bet)
        }).collect();

    type_hand_bets.sort();

    type_hand_bets.into_iter()
    //.inspect(|x| println!("{:?}", x))
        .map(|(_, _, bet)| bet)
        .enumerate()
        .fold(0, |acc, (idx, bet)| acc + (idx as Bet + 1) * bet)
}

pub fn part_two(input: &str) -> Result<String, String> {
    let mut hand_bets = parse_input(input)
        .map_err(|_| "trouble parsing input".to_string())?;

    hand_bets.iter_mut()
        .for_each(|(hand, _)| hand.jokerify());

    let answer = solve(&hand_bets);

    Ok(answer.to_string())
}

const _EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

const _ANSWER: &str = "6440";
#[test]
fn test_part_one() {
    assert_eq!(_ANSWER, &part_one(_EXAMPLE).unwrap());
}

#[test]
fn test_hand_type() {
    let hand: Hand = "AAQ3Q".parse().expect("hand parsing failed");
    assert_eq!(hand.find_type(), HandType::TwoPair)
}

#[test]
fn test_part_two() {
    // assert_eq!(_ANSWER, &part_two(_EXAMPLE).unwrap());
}