use std::collections::HashMap;

use nom::{branch::alt, bytes::complete::{tag, take_while_m_n}, character::complete::newline, combinator::map, multi::{fold_many0, many1}, sequence::{delimited, separated_pair, terminated}, IResult, Parser};

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

fn node_ident(input: &str) -> IResult<&str, &str> {
    take_while_m_n(3, 3, char::is_alphanumeric)(input)
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

/*
I think a solution to this problem is going to have to do with periodicity.
In  the example, both "ghosts" get into cycles within the first few steps.
One cycle has a period of 2, and the other 3.
Whether you re-enter a previously encountered cycle will have everything to do with both which node you are at and where the instructions are up to.

*/

pub fn part_two(input: &str) -> Result<String, String> {
    let (_, (instructions, node_tree)) = parse_input(input)
        .map_err(|e| e.to_string())?;

    let mut ghosts = nodes_ending_with(&node_tree, 'A');
    //println!("{ghosts:?}");
    let mut counter = 0;

    let mut rl_iter = instructions.into_iter().cycle();

    while !all_z(&ghosts) {
        counter += 1;
        let next_instr = rl_iter.next().unwrap();
        ghosts.iter_mut()
            .for_each(|node_name| {
                *node_name = node_tree.get(node_name)
                .unwrap()
                .next(next_instr);
            });
    }
    
    Ok(counter.to_string())
}

fn all_z(node_names: &[&str]) -> bool {
    node_names.iter()
        .all(|name| name.ends_with('Z'))
}

fn nodes_ending_with<'a>(
    nodes: &HashMap<&'a str, Node<'a>>,
    ending: char,
) -> Vec<&'a str> {
    nodes
        .values()
        .filter_map(|node| {
            if node.name.ends_with(ending) {
                Some(node.name)
            } else {
                None
            }
        })
        .collect()
}

const _EXAMPLE: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const _ANSWER: &str = "6";

const _EXAMPLE_2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

const _ANSWER_2: &str = "6";

#[test]
fn test_part_one() {
    assert_eq!(_ANSWER, &part_one(_EXAMPLE).unwrap());
}

#[test]
fn test_part_two() {
    assert_eq!(_ANSWER_2, &part_two(_EXAMPLE_2).unwrap());
}