fn parse_single_line(line: &str, ret: &mut Vec<Vec<char>>)
{
    let mut chars = line.chars();
    let mut c = chars.nth(1);
    for ret_vec in ret
    {

        if c != Some(' ')
        {
            ret_vec.insert(0, c.unwrap());
        }
        c = chars.nth(3);
    }
}

fn parse_start<'a, I>(lines: &mut I) -> Vec<Vec<char>> where I: Iterator<Item = &'a str>
{
    let first_line = lines.next().unwrap();
    if first_line.is_empty()
    {
        return parse_start(lines);
    }
    let num_lines = (first_line.len() + 1)/4;
    let mut ret: Vec<Vec<char>> = Vec::with_capacity(num_lines);

    for _ in 0..num_lines
    {
        ret.push(vec![]);
    }

    parse_single_line(first_line, &mut ret);
    for line in lines
    {
        if line.chars().nth(1) == Some('1')
        {
            break;
        }
        parse_single_line(line, &mut ret);
    }

    ret
}

fn modify(state: &mut Vec<Vec<char>>, from: usize, to: usize)
{
    let c = state[from].pop().unwrap();
    state[to].push(c);
}

fn modify_mult(state: &mut Vec<Vec<char>>, from: usize, to: usize, count: usize)
{
    let split_at = state[from].len() - count;
    let mut c = state[from].split_off(split_at);
    assert!(c.len() == count);
    state[to].append(&mut c);
}

fn parse_instructions<'a, I>(lines: &mut I, state: &mut Vec<Vec<char>>, is_part_2: bool) where I: Iterator<Item = &'a str>
{
    for line in lines
    {
        if line.is_empty()
        {
            break;
        }
        let mut split = line.split(' ');
        let count : usize = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from : usize = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to : usize = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        if !is_part_2
        {
            for _ in 0..count
            {
                modify(state, from, to);
            }
        }
        else
        {
            modify_mult(state, from, to, count);
        }
    }
}

pub fn door_5_1(lines: &str) -> String
{
    let mut l_iter = lines.lines();
    let mut state = parse_start(&mut l_iter);
    l_iter.next();
    parse_instructions(&mut l_iter, &mut state, false);

    let mut res = String::from("");
    for mut s in state
    {
        res.push(s.pop().unwrap());
    }

    res
}

pub fn door_5_2(lines: &str) -> String
{
    let mut l_iter = lines.lines();
    let mut state = parse_start(&mut l_iter);
    l_iter.next();
    parse_instructions(&mut l_iter, &mut state, true);

    let mut res = String::from("");
    for mut s in state
    {
        res.push(s.pop().unwrap());
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        let input =
"[N]             [R]             [C]
[T] [J]         [S] [J]         [N]
[B] [Z]     [H] [M] [Z]         [D]
[S] [P]     [G] [L] [H] [Z]     [T]
[Q] [D]     [F] [D] [V] [L] [S] [M]
[H] [F] [V] [J] [C] [W] [P] [W] [L]
[G] [S] [H] [Z] [Z] [T] [F] [V] [H]
[R] [H] [Z] [M] [T] [M] [T] [Q] [W]";

        let mut lines = input.lines();
        let result = parse_start(&mut lines);
        assert_eq!(result[0].len(), 8);
        assert_eq!(result[7].len(), 4);
        assert_eq!(result[0][0], 'R');
    }

    #[test]
    fn part_1_works() {
        let input =
"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

";

        let result = door_5_1(input);
        assert_eq!(result, String::from("CMZ"));
    }
    
    #[test]
    fn part_2_works() {
        let input =
"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

";

        let result = door_5_2(input);
        assert_eq!(result, String::from("MCD"));
    }
}
