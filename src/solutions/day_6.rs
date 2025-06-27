pub fn part_one(input: &str) -> Result<String, String> {
    let races = parse_input(input).ok_or("trouble parsing input".to_string())?;
    let answer = solve(&races);

    Ok(answer.to_string())
}

struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        // x * (time - x) > distance [how many options for x?]
        // x * (t - x) = d + 1 [shoulder values] 
        // x^2 - tx + (d+1) = 0 [solve quadratic]
        // ways = floor(x2) - ceil(x1) + 1

        let t = self.time as f64;
        let d = self.dist as f64;

        let discriminant = (t*t - 4.0 * (d + 1.0)).sqrt();
        let x1 = (t - discriminant) / 2.0;
        let x2 = (t + discriminant) / 2.0;
        
        x2.floor() as u64 - x1.ceil() as u64 + 1
    }
}

fn parse_input(input: &str) -> Option<Vec<Race>> {
    let (line1, line2) = input.split_once('\n')?;

    let to_nums = |input: &str| -> Option<Vec<u64>> {
        input.split_whitespace()
            .skip(1)
            .map(|num| num.parse().ok())
            .collect()
    };

    let times = to_nums(line1)?;
    let dists = to_nums(line2)?;

    let races = times.into_iter().zip(dists.into_iter())
        .map(|(time, dist)| {
            Race {
                time,
                dist
            }
        })
        .collect();

    Some(races)
}

fn solve(races: &[Race]) -> u64 {
    races.into_iter()
        .fold(1, |acc, race| acc * race.ways_to_win())
}

pub fn part_two(input: &str) -> Result<String, String> {
    let race = parse_input_2(input).ok_or("trouble parsing input".to_string())?;
    let answer = race.ways_to_win();

    Ok(answer.to_string())
}

fn parse_input_2(input: &str) -> Option<Race> {
    let (line1, line2) = input.split_once('\n')?;

    let to_num = |input: &str| -> Option<u64> {
        let digits = input.split_whitespace()
            .skip(1)
            .collect::<String>();
        digits.parse().ok()
    };

    let time = to_num(line1)?;
    let dist = to_num(line2)?;

    Some(
        Race {
            time,
            dist,
        }
    )
}

const _EXAMPLE: &str = "\
";

const _ANSWER: &str = "\
";

#[test]
fn test_part_one() {
    // assert_eq!(_ANSWER, &part_one(_EXAMPLE).unwrap());
}

#[test]
fn test_part_two() {
    // assert_eq!(_ANSWER, &part_two(_EXAMPLE).unwrap());
}