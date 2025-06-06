/* 
There's a subtle issue with part two I ran into; "fiveight" should become 58 but a left-to-right parser will take "five" and won't process the remaining "ight" properly. 

The nicest solution would be to write a function that runs until it finds the first occurence of a match from an array of substrings, then returns the index of the substring it found. That way the string is only traversed once. Then, ideally we could enter the input string and substrings into the function backwards to find the last occurrence.
*/

pub fn part_one(input: &str) -> Result<String, String> {
    let sum: u32 = input.lines()
        .map(|line| {
            line_to_num(line)
        })
        .sum::<Result<u32, String>>()?;

    Ok(sum.to_string())
}

fn line_to_num(input: &str) -> Result<u32, String> {
    let mut digits = input.chars()
        .filter_map(|c| c.to_digit(10));

    let first = digits.next().ok_or("Line contained no digits")?;
    let last = digits.rev().next().unwrap_or(first);

    return Ok(10 * first + last);
}

pub fn part_two(input: &str) -> Result<String, String> {
    let sum: u32 = input.lines()
        .map(|line| {
            parse_line(line)
        })
        .sum();

    Ok(sum.to_string())
}

const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn match_first(input: &str, pats: &[&str]) -> Option<usize> {
    // returns the index of the first pattern that matched, or None if no pattern matched

    let mut match_idx = None;
    let mut pat_idx = None;

    for (idx, pat) in pats.iter().enumerate() {
        let this_match = input.find(pat);
        match (match_idx, this_match) {
            (None, Some(_)) => {
                match_idx = this_match;
                pat_idx = Some(idx);
            }
            (Some(x), Some(y)) if y < x => {
                match_idx = this_match;
                pat_idx = Some(idx);
            }
            _ => {}
        }
    }

    return pat_idx;
}

// absolutely terrible way to do this but it's the first thing I thought of
fn match_last(input: &str, pats: &[&str]) -> Option<usize> {
    // returns the index of the first pattern that matched, or None if no pattern matched

    let mut match_idx = None;
    let mut pat_idx = None;

    for (idx, pat) in pats.iter().enumerate() {
        let this_match = input.rfind(pat);
        if this_match > match_idx {
            match_idx = this_match;
            pat_idx = Some(idx);
        }
    }

    return pat_idx;
} 

fn parse_line(input: &str) -> u32 {
    let pats: &[&str] = &[DIGITS, WORDS].concat();

    let first_idx = match_first(input, pats).expect("should always match");
    let last_idx = match_last(input, pats).expect("should always match");

    return ((first_idx % 9 + 1) * 10 + (last_idx % 9 + 1)) as u32;
}

const _EXAMPLE_1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

const _ANSWER_1: &str = "142";

const _EXAMPLE_2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

const _ANSWER_2: &str = "281";

#[test]
fn test_part_one() {
    assert_eq!(&part_one(_EXAMPLE_1).unwrap(), _ANSWER_1);
}

#[test]
fn test_part_two() {
    assert_eq!(&part_two(_EXAMPLE_2).unwrap(), _ANSWER_2);
    assert_eq!(&part_two("vsqsxgqxn8xkxftpmtrtssxgnfqcqdnsixdsxhhxgonefive").unwrap(), "85");
    // assert_eq!(&part_two("1fiveight").unwrap(), "18");
}

#[test]
fn test_match_first() {
    let input = "1fiveightt";
    assert_eq!(match_first(input, &[DIGITS, WORDS].concat()), Some(0));
}

#[test]
fn test_match_last() {
    let input = "1fiveightt";
    assert_eq!(match_last(input, &[DIGITS, WORDS].concat()), Some(16));
}