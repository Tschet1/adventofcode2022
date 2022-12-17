fn char_to_mask_index(c: char) -> u8
{
    if c.is_ascii_lowercase()
    {
        return ((c as u32) - 97 + 1) as u8
    }
    else if c.is_ascii_uppercase()
    {
        return ((c as u32) - 65 + 26 + 1) as u8
    }
    else
    {
        panic!("Is no letter");
    }

}

fn first_bag_to_mask<I>(chars: I) -> u64 where I: Iterator<Item = char>
{
    let mut mask : u64 = 0;
    for c in chars
    {
        mask |= 1 << char_to_mask_index(c);
    }

    mask
}

fn check_second_bag<I>(chars: I, mask: u64) -> u32 where I: Iterator<Item = char>
{
    for c in chars
    {
        let mask_index = char_to_mask_index(c);
        if mask & 1<<mask_index != 0
        {
            /* Found the duplicate */
            return mask_index as u32;
        }
    }
    panic!("No duplicates");
}

fn check_bag_pair(input: &str) -> u32
{
    let chars = input.chars();

    let bag_size = chars.count()/2;
    let mask = first_bag_to_mask(input[..bag_size].chars());
    check_second_bag(input[bag_size..].chars(), mask)
}

fn check_bag_tripple(bag1: &str, bag2: &str, bag3: &str) -> u32
{
    let mask = first_bag_to_mask(bag1.chars());
    let mask = mask & first_bag_to_mask(bag2.chars());
    check_second_bag(bag3.chars(), mask)
}

pub fn door_3_1(input: &str) -> u32
{
    let mut result : u32 = 0;
    for line in input.lines()
    {
        if !line.is_empty()
        {
            result += check_bag_pair(line);
        }
    }

    result
}

pub fn door_3_2(input: &str) -> u32
{
    let mut result : u32 = 0;
    let mut lines = input.lines().filter(|s| !s.is_empty()).peekable();
    while lines.peek().is_some()
    {
        let l1 = lines.next().unwrap();
        let l2 = lines.next().unwrap();
        let l3 = lines.next().unwrap();
        result += check_bag_tripple(l1, l2, l3);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
let input = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        let result = door_3_1(input);
        assert_eq!(result, 157);
    }

    #[test]
    fn part_2_works() {
let input = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        let result = door_3_2(input);
        assert_eq!(result, 70);
    }

    #[test]
    fn part_2_works_part() {
let input = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
";
        let result = door_3_2(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn part_2_simple() {
let input = "
aA
bA
abAc
";
        let result = door_3_2(input);
        assert_eq!(result, 27);
    }
}
