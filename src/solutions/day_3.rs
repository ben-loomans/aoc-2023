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

    fn is_part_num(&self, part_num: &PartNum) -> bool {
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

    fn part_num_to_usize(&self, part_num: &PartNum) -> usize {
        let num_str = &self.text[part_num.idx..part_num.idx + part_num.len];
        num_str.parse().expect("this conversion should always work")
    }
}

struct PartIter<'a> {
    schematic: &'a Schematic<'a>,
    idx: usize,
}

impl<'a> IntoIterator for &'a Schematic<'a> {
    type Item = PartNum;

    type IntoIter = PartIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PartIter {
            schematic: self,
            idx: 0,
        }
    }
}

impl<'a> Iterator for PartIter<'a> {
    type Item = PartNum;

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

        let part_num = PartNum {
            idx: start_idx, 
            len: end_idx - start_idx,
        };

        Some(part_num)
    }
}

struct PartNum {
    idx: usize,
    len: usize,
}

pub fn part_two(input: &str) -> Result<String, String> {
    Err("Unimplemented".to_string())
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

#[test]
fn test_part_one() {
    assert_eq!(_ANSWER, &part_one(_EXAMPLE).unwrap());
}

#[test]
fn test_part_two() {
    // assert_eq!(_ANSWER, &part_two(_EXAMPLE).unwrap());
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