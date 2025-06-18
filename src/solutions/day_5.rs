pub fn part_one(input: &str) -> Result<String, String> {
    Err("Unimplemented".to_string())
}

pub fn part_two(input: &str) -> Result<String, String> {
    Err("Unimplemented".to_string())
}

#[test]
fn test_part_one() {
    // assert_eq!(_ANSWER, &part_one(_EXAMPLE).unwrap());
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