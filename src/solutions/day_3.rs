use std::collections::BTreeMap;

/*
this solution is quite a bit more involved than it needs to be, 
I just wanted to explore solving the problem without allocating.
Currently solves in about 2.2ms on my computer.
*/
pub fn part_one(input: &str) -> Result<String, String> {
    let schematic = Schematic::new(input);
    let sum: usize = schematic.into_iter()
        .filter(|part_num| {
            schematic.is_part_num(part_num)
        }).map(|part_num| {
            schematic.part_num_to_usize(&part_num)
        }).sum();

    Ok(sum.to_string())
}


struct Schematic<'a> {
    text: &'a str,
    width: usize,
}

impl<'a> Schematic<'a> {
    fn new(text: &'a str) -> Self {
        let width = text.find('\n').unwrap_or(text.len());

        Self {
            text, 
            width,
        }
    }

    fn is_part_num(&self, part_num: &Num) -> bool {
        let (x, y) = self.idx_to_xy(part_num.idx).expect("part num idx always good");

        for j in (y - 1)..=(y + 1) { 
            for i in (x - 1)..=(x + (part_num.len as i32)) { 
                if self.is_symbol(i, j) {
                    return true;
                }
            }
        }

        return false;
    }



    fn idx_to_xy(&self, idx: usize) -> Option<(i32, i32)> {
        if idx >= self.text.len() {
            return None;
        }

        let y = idx / (self.width + 1); // +1 for newline
        let x = idx % (self.width + 1);

        return Some((x as i32, y as i32));
    }

    fn is_symbol(&self, x: i32, y: i32) -> bool {
        // println!("checking ({x} ,{y})...");
        if let Some(idx) = self.xy_to_idx(x, y) {
            match self.text.as_bytes()[idx] as char {
                '.' => return false,
                x if x.is_ascii_digit() => return false,
                _ => return true,
            }
        }

        return false;
    }

    fn xy_to_idx(&self, x: i32, y: i32) -> Option<usize> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        let idx = (self.width + 1) * y + x;

        if x >= self.width {
            return None;
        }

        if idx > self.text.len() {
            return None;
        }
        
        return Some(idx);
    }

    fn part_num_to_usize(&self, part_num: &Num) -> usize {
        let num_str = &self.text[part_num.idx..part_num.idx + part_num.len];
        num_str.parse().expect("this conversion should always work")
    }

    // part 2

    fn is_gear(&self, x: i32, y: i32) -> bool {
        if let Some(idx) = self.xy_to_idx(x, y) {
            return self.text.as_bytes()[idx] == b'*';
        }

        return false;
    }

    fn find_gears(&self, part_num: &Num) -> Vec<usize> {
        let mut gears = Vec::new();

        let (x, y) = self.idx_to_xy(part_num.idx).expect("part num idx always good");

        for j in (y - 1)..=(y + 1) { 
            for i in (x - 1)..=(x + (part_num.len as i32)) { 
                if self.is_gear(i, j) {
                    let gear = self.xy_to_idx(i, j).expect("Conversion shouldn't fail");
                    gears.push(gear);
                }
            }
        }

        gears
    }
}

struct NumIter<'a> {
    schematic: &'a Schematic<'a>,
    idx: usize,
}

impl<'a> IntoIterator for &'a Schematic<'a> {
    type Item = Num;

    type IntoIter = NumIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NumIter {
            schematic: self,
            idx: 0,
        }
    }
}

impl<'a> Iterator for NumIter<'a> {
    type Item = Num;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.schematic.text.len() {
            return None;
        }

        let relative_start = self.schematic.text[self.idx..]
            .find(|c: char| c.is_digit(10))?;

        let start_idx = self.idx + relative_start;

        let relative_end = self.schematic.text[start_idx..]
            .find(|c: char| !c.is_digit(10))
            .unwrap_or(self.schematic.text.len() - start_idx);

        let end_idx = start_idx + relative_end;

        self.idx = end_idx + 1;     // next search should start from the next unchecked char

        let part_num = Num {
            idx: start_idx, 
            len: end_idx - start_idx,
        };

        Some(part_num)
    }
}

struct Num {
    idx: usize,
    len: usize,
}

/*
For part two I'm not going to worry about not allocating. 
I think parsing the input to a set of '*'s and a set of numbers might be a good start
Then, scan the surrounds of each number to find a gear and save the gear-number pair.

*/

pub fn part_two(input: &str) -> Result<String, String> {
    let schematic = Schematic::new(input);
    let gear_list = GearList::from_schematic(&schematic);
    let sum = gear_list.sum();
        
    Ok(sum.to_string())
}

struct GearList(BTreeMap<usize, Vec<usize>>);

impl GearList {
    fn new() -> Self {
        Self(
            BTreeMap::new()
        )
    }

    fn from_schematic(schematic: &Schematic) -> Self {
        let mut gear_list = GearList::new();

        schematic.into_iter()
            .map(|num| {
                let part_num = schematic.part_num_to_usize(&num);
                let gears = schematic.find_gears(&num);
                
                gears.into_iter()
                    .map(move |gear| {
                        (gear, part_num)
                    })
            }).flatten()
            .for_each(|(gear, num)| {
                gear_list.insert(gear, num);
            });

        gear_list
    }

    fn insert(&mut self, gear_idx: usize, num: usize) {
        self.0.entry(gear_idx)
        .or_insert(Vec::new())
        .push(num);
    }

    fn sum(&self) -> usize {
        self.0.values()
            .filter_map(|nums| {
                if nums.len() != 2 {
                    return None;
                }

                Some(nums[0] * nums[1])
            }).sum()
    }
}

const _EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

const _ANSWER: &str = "4361";
const _ANSWER_2: &str = "467835";

#[test]
fn test_part_one() {
    assert_eq!(_ANSWER, &part_one(_EXAMPLE).unwrap());
}

#[test]
fn test_part_two() {
    assert_eq!(_ANSWER_2, &part_two(_EXAMPLE).unwrap());
}

#[test]
fn test_symbol() {
    let schematic = Schematic::new(_EXAMPLE);

    assert!(schematic.is_symbol(3, 1));
    assert!(!schematic.is_symbol(3, 2));
}

#[test]
fn test_is_part() {
    let schematic = Schematic::new(_EXAMPLE);
    let mut part_iter = schematic.into_iter();

    let first = part_iter.next().unwrap();
    assert!(schematic.is_part_num(&first));

    let second = part_iter.next().unwrap();
    assert!(!schematic.is_part_num(&second));

}