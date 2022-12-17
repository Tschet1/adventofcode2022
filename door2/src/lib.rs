fn score_from_game(other: &char, me: &char) -> u8
{
    let mut score : u8 = 0;

    score += {
        match me {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Unknown sign")
        }
    };

    score += {
        match (me, other) {
            ('X', 'A') | ('Y', 'B') | ('Z', 'C') => 3,
            ('Y', 'A') | ('Z', 'B') | ('X', 'C') => 6,
            ('Z', 'A') | ('X', 'B') | ('Y', 'C') => 0,
            _ => panic!("Unknown sign")
        }
    };

    score
}

fn score_from_game_variant_2(other: &char, me: &char) -> u8
{
    let new_me: char = match (me, other) {
        ('X', 'A') => 'Z',
        ('X', 'B') => 'X',
        ('X', 'C') => 'Y',
        ('Y', 'A') => 'X',
        ('Y', 'B') => 'Y',
        ('Y', 'C') => 'Z',
        ('Z', 'A') => 'Y',
        ('Z', 'B') => 'Z',
        ('Z', 'C') => 'X',
        _ => panic!("Unknown sign")
    };

    score_from_game(other, &new_me)
}

fn line_to_score(input: &str) -> u8
{
    let mut chars = input.chars();
    score_from_game(&chars.next().unwrap(), &chars.nth(1).unwrap())
}

fn line_to_score_variant_2(input: &str)-> u8
{
    let mut chars = input.chars();
    score_from_game_variant_2(&chars.next().unwrap(), &chars.nth(1).unwrap())
}

pub fn door_2_1(input: &str) -> u32
{
    let mut result : u32 = 0;
    for line in input.lines()
    {
        if !line.is_empty()
        {
            result += line_to_score(line) as u32;
        }
    }

    result
}

pub fn door_2_2(input: &str) -> u32
{
    let mut result : u32 = 0;
    for line in input.lines()
    {
        if !line.is_empty()
        {
            result += line_to_score_variant_2(line) as u32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_works() {
        let result = score_from_game(&'A', &'Y');
        assert_eq!(result, 8);

        let result = score_from_game(&'B', &'X');
        assert_eq!(result, 1);

        let result = score_from_game(&'C', &'Z');
        assert_eq!(result, 6);
    }

    #[test]
    fn single_game_works() {
        let result = line_to_score(&"A Y");
        assert_eq!(result, 8);
    }

    #[test]
    fn part1_works() {
        let result = door_2_1(&"
A Y
B X
C Z
");
        assert_eq!(result, 15);
    }

    #[test]
    fn part2_works() {
        let result = door_2_2(&"
A Y
B X
C Z
");
        assert_eq!(result, 12);
    }
}
