use std::{ops::Range, str::FromStr};

pub fn part_one(input: &str) -> Result<String, String> {
    let (seeds, maps) = input.split_once("\n\n")
        .ok_or("bad input".to_string())?;

    let mut seeds = parse_seeds(seeds)?;
    let maps = parse_maps(maps)?;

    for map in &maps {
        for seed in &mut seeds {
            *seed = map.map(*seed);
        }
    }

    seeds.sort();

    Ok(seeds[0].to_string())
}

fn parse_seeds(input: &str) -> Result<Vec<usize>, String> {
    let parse = |input: &str| -> Option<Vec<usize>> {
        let (_, nums) = input.split_once(": ")?;
        nums.split(' ')
            .map(|num| num.parse().ok())
            .collect::<Option<Vec<usize>>>() 
    };

    parse(input).ok_or_else(|| {
            format!("Couldn't parse Vec<usize> from input `{input}`")
        })
}

fn parse_maps(input: &str) -> Result<Vec<RangeMap>, String> {
    let maps = input.split("\n\n")
        .map(|lines| {
            lines.parse()
        })
        .collect::<Result<Vec<RangeMap>, String>>()?;

    Ok(maps)
}

struct RangeMap {
    mappings: Vec<Mapping>,
}

impl RangeMap {
    fn map(&self, num: usize) -> usize {
        let idx = match self.mappings.binary_search_by_key(&num, |m| {
            m.source.start
        }) {
            Ok(exact) => exact,
            Err(next) => next.saturating_sub(1),
        };

        self.mappings[idx].map(num)
    }
}

impl FromStr for RangeMap {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mappings = s.lines()
            .skip(1) // title line
            .map(|line| {
                line.parse()
            })
            .collect::<Result<Vec<Mapping>, String>>()?;
        mappings.sort();

        Ok(RangeMap { mappings })
    }
}

#[derive(Debug, Eq)]
struct Mapping {
    source: Range<usize>,
    offset: isize,
}

impl Mapping {
    fn map(&self, num: usize) -> usize {
        if !self.source.contains(&num) {
            return num;
        }

        num.saturating_add_signed(self.offset)
    }
    
    #[inline(always)]
    fn start(&self) -> usize {
        self.source.start
    }
}

impl Ord for Mapping {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source.start.cmp(&other.source.start)
    }
}

impl PartialOrd for Mapping {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.source.start.partial_cmp(&other.source.start)
    }
}

impl PartialEq for Mapping {
    fn eq(&self, other: &Self) -> bool {
        self.source.start == other.source.start
    }
}

impl FromStr for Mapping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse = |s: &str| -> Option<Self> {
            let mut nums = s.split(' ');
            let (dst, src, len): (usize, usize, usize) = (
                nums.next()?.parse().ok()?,
                nums.next()?.parse().ok()?,
                nums.next()?.parse().ok()?,
            );
            
            let parsed = Self {
                source: src..src+len,
                offset: dst as isize - src as isize,
            };

            Some(parsed)
        };

        parse(s).ok_or_else(|| {
            format!("Couldn't parse Mapping from input `{s}`")
        })
    }
}

pub fn part_two(input: &str) -> Result<String, String> {
    Err("Unimplemented".to_string())
}

impl Mapping {
    // applies self mapping to other mapping - can create multiple new mappings
    fn map_mapping(&self, other: &Mapping) -> Vec<Mapping> {
        if self.source.start >= other.source.end ||
            self.source.end < other.source.start
        {
            // ranges don't overlap
        }
    }
}

#[test]
fn test_part_one() {
    assert_eq!(_ANSWER, &part_one(_EXAMPLE).unwrap());
}

#[test]
fn test_mapping_from_str() {
    let parsed = "3998185854 3762642503 103735883".parse();
    let mapping = Mapping {
        source: 3762642503..3866378386,
        offset: 235543351,
    };

    assert_eq!(parsed, Ok(mapping))
}

#[test]
fn test_part_two() {
    // assert_eq!(_ANSWER, &part_two(_EXAMPLE).unwrap());
}

const _EXAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

const _ANSWER: &str = "35";