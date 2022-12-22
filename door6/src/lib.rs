fn find_start(line: &str, sequence_length: usize) -> usize
{
    let mut marker: Vec<char> = vec!['.'; sequence_length];

    'outer: for (count, c) in line.chars().enumerate()
    {
        marker[count % sequence_length] = c;

        for i in 0..sequence_length
        {
            for j in i+1..sequence_length
            {
                if count < sequence_length || marker[i] == marker[j]
                {
                    continue 'outer;
                }
            }
        }
        return count + 1;
    }
    return 0;
}

pub fn door_6_1(lines: &str) -> usize
{
    for line in lines.lines()
    {
        if line.is_empty() {
            continue;
        }

        return find_start(line, 4);
    }
    return 0;
}

pub fn door_6_2(lines: &str) -> usize
{
    for line in lines.lines()
    {
        if line.is_empty() {
            continue;
        }

        return find_start(line, 14);
    }
    return 0;
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(door_6_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(door_6_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(door_6_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(door_6_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(door_6_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(door_6_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(door_6_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(door_6_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(door_6_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(door_6_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
