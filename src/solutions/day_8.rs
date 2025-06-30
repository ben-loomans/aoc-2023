use std::{collections::{HashMap, HashSet}, hash::Hash};

use nom::{branch::alt, bytes::complete::{tag, take_until, take_while_m_n}, character::{char, complete::newline}, combinator::map, multi::{count, fold_many0, many1, many_m_n, many_till}, sequence::{delimited, separated_pair, terminated}, IResult, Parser};

pub fn part_one(input: &str) -> Result<String, String> {
    let (_, (instructions, node_tree)) = parse_input(input)
        .map_err(|e| e.to_string())?;
    
    let mut current = "AAA";
    let end = "ZZZ";
    let mut counter = 0;

    let mut rl_iter = instructions.into_iter().cycle();

    while current != end {
        counter += 1;
        let next_instr = rl_iter.next().unwrap();
        current = node_tree.get(current)
            .unwrap()
            .next(next_instr);
    }

    Ok(counter.to_string())
}

fn parse_rl(input: &str) -> IResult<&str, Vec<bool>> {
    terminated(
        many1(
            alt((
                map(tag("R"), |_| true),
                map(tag("L"), |_| false),
            ))
        ),
        newline
    ).parse(input)
}

fn is_uppercase(c: char) -> bool {
    c.is_ascii_uppercase()
}

fn node_ident(input: &str) -> IResult<&str, &str> {
    take_while_m_n(3, 3, is_uppercase)(input)
}

fn parse_node<'a>(input: &'a str) -> IResult<&'a str, Node<'a>> {
    let (rest, (name, (left, right))) = separated_pair(
        node_ident, 
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(
                node_ident, 
                tag(", "),
                node_ident 
            ),
            tag(")"),
        )
    ).parse(input)?;

    let node = Node {
        name,
        left,
        right
    };

    Ok((rest, node))
}

fn parse_nodes<'a>(input: &'a str) -> IResult<&'a str, HashMap<&'a str, Node<'a>>> {
    fold_many0(
        terminated(parse_node, newline), 
        HashMap::new,
        |mut acc, node| {
            acc.insert(node.name, node); 
            acc
        }     
    ).parse(input)
}

fn parse_input<'a>(input: &'a str) -> IResult<&'a str, (Vec<bool>, HashMap<&'a str, Node<'a>>)> {
    separated_pair(
        parse_rl,
        newline,
        parse_nodes
    ).parse(input)
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    fn next(&self, right: bool) -> &'a str {
        if right {
            self.right
        } else {
            self.left
        }
    }
}

pub fn part_two(input: &str) -> Result<String, String> {
    Err("Unimplemented".to_string())
}

const _EXAMPLE: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const _ANSWER: &str = "6";
#[test]
fn test_part_one() {
    assert_eq!(_ANSWER, &part_one(_EXAMPLE).unwrap());
}

#[test]
fn test_part_two() {
    // assert_eq!(_ANSWER, &part_two(_EXAMPLE).unwrap());
}