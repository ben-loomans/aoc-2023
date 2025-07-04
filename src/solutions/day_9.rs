use std::ops::Sub;
/* TODO
    clean up the unnecessarily abstract Diff code
    OR
    make all of it abstract and iterator-based (no allocation)
*/

pub fn part_one(input: &str) -> Result<String, String> {
    let answer: i32 = input.lines()
        .map(|line| {
            let nums: Vec<i32> = line.split(' ')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            extrapolate(&nums, nums.len() as i32)
        }).sum();

    Ok(answer.to_string())
}

// ideally this function wouldn't have to allocate
fn find_coeffs(input: &[i32]) -> Vec<i32> {

    let mut diffs = input.to_owned();
    let mut coeffs = Vec::new();

    loop {
        if diffs.iter().all(|diff| *diff == 0) {
            break;
        }

        coeffs.push(diffs[0]);
        diffs = diffs.into_iter().diff().collect();
    }

    coeffs
}

fn extrapolate(input: &[i32], x: i32) -> i32 {
    let d = find_coeffs(input);
    let k = d.len() as i32;

    let mut c = 1;
    let mut sum = 0;

    for i in 0..k {
        sum += c * d[i as usize];
        c = c * (x - i) / (i + 1);
    }
    
    sum
}

// An iterator adaptor that yields the difference between each pair of consecutive items.
pub struct Diff<I>
where
    I: Iterator,
{
    iter: I,
    prev: Option<I::Item>,
}

impl<I> Diff<I>
where
    I: Iterator,
    I::Item: Copy,
{
    // Create a `Diff` from an iterator by consuming its first element as the initial `prev`.
    // Returns `None` if the iterator was empty.
    pub fn new(mut iter: I) -> Self {
        let prev = iter.next();
        
        Diff { 
            iter, 
            prev,
        }
    }
}

impl<I> Iterator for Diff<I>
where
    I: Iterator,
    I::Item: Copy + Sub<Output = I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.prev {
            None => return None,
            Some(prev) => {
                let next = self.iter.next();
                self.prev = next;

                let delta = next? - prev;
                Some(delta)
            }
        }
    }
}

// Extension trait so we can just write `foo.iter().diff()`
pub trait IteratorDiff: Iterator + Sized
where
    Self::Item: Copy + Sub<Output = Self::Item>,
{
    fn diff(self) -> Diff<Self> {
        Diff::new(self)
    }
}

impl<I> IteratorDiff for I
where
    I: Iterator,
    I::Item: Copy + Sub<Output = I::Item>,
{}


pub fn part_two(input: &str) -> Result<String, String> {
    let answer: i32 = input.lines()
        .map(|line| {
            let nums: Vec<i32> = line.split(' ')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            extrapolate(&nums, -1)
        }).sum();

    Ok(answer.to_string())
}

const _EXAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

const _ANSWER: &str = "114";
const _ANSWER_2: &str = "2";

#[test]
fn test_part_one() {
    assert_eq!(_ANSWER, &part_one(_EXAMPLE).unwrap());
}

#[test]
fn test_coeffs() {
    let nums = [10, 13, 16, 21, 30, 45];
    let coeffs = find_coeffs(&nums);
    assert_eq!(coeffs, vec![10, 3, 0, 2]);
}

#[test]
fn test_extrapolate() {
    let nums = [10, 13, 16, 21, 30, 45];
    let next = extrapolate(&nums, nums.len() as i32);

    assert_eq!(next, 68);
}

#[test]
fn test_part_two() {
    assert_eq!(_ANSWER_2, &part_two(_EXAMPLE).unwrap());
}