use std::str::FromStr;
use std::num::ParseIntError;

struct Range {
    lower: u32,
    upper: u32
}

impl Range {
    fn contains(&self, r2: &Range) -> bool
    {
        self.lower <= r2.lower && self.upper >= r2.upper
    }

    fn overlap(&self, r2: &Range) -> bool
    {
        (self.lower <= r2.lower && self.upper >= r2.lower) || (self.lower <= r2.upper && self.upper >= r2.upper)
    }
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, u) = s.split_once("-").unwrap();

        let l = l.parse::<u32>()?;
        let u = u.parse::<u32>()?;

        Ok(Range { lower: l, upper: u })
    }
}

fn check_pair(line: &str) -> bool
{
    let (a,b) = line.split_once(",").unwrap();
    let a = Range::from_str(a).unwrap();
    let b = Range::from_str(b).unwrap();

    a.contains(&b) || b.contains(&a)
}

fn check_pair_overlap(line: &str) -> bool
{
    let (a,b) = line.split_once(",").unwrap();
    let a = Range::from_str(a).unwrap();
    let b = Range::from_str(b).unwrap();

    a.overlap(&b) || b.overlap(&a)
}

pub fn door_4_1(input: &str) -> u32
{
    let mut res = 0;
    for line in input.lines()
    {
        if !line.is_empty() && check_pair(line)
        {
            res += 1;
        }
    }
    res
}

pub fn door_4_2(input: &str) -> u32
{
    let mut res = 0;
    for line in input.lines()
    {
        if !line.is_empty() && check_pair_overlap(line)
        {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn CreateRange() {
        let result = Range::from_str("123-999").unwrap();

        assert_eq!(result.lower, 123);
        assert_eq!(result.upper, 999);
    }

    #[test]
    fn checkContained() {
        let result = check_pair("123-999,123-888");
        assert_eq!(result, true);
    }

    #[test]
    fn checkContained2() {
        let result = check_pair("124-999,123-999");
        assert_eq!(result, true);
    }

    #[test]
    fn checknotcontained() {
        let result = check_pair("123-999,122-888");
        assert_eq!(result, false);
    }

    #[test]
    fn part_1_works() {
        let result = door_4_1("
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
");

        assert_eq!(result, 2);
    }

    #[test]
    fn part_2_works() {
        let result = door_4_2("
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
");

        assert_eq!(result, 4);
    }
}
