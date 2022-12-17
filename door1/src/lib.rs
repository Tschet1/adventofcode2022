
fn parse(input: &str) -> Vec<Vec<u32>>{
    let mut res : Vec<Vec<u32>> = vec![];

    let mut elv : Vec<u32> = vec![];
    for line in input.lines()
    {
        match line.trim() {
            "" => {
                if !elv.is_empty()
                {
                    res.push(elv);
                    elv = vec![];
                }
            },
            l => {
                let num : u32 = l.parse::<u32>().unwrap();
                elv.push(num);
            }
        }
    }
    if !elv.is_empty()
    {
        res.push(elv);
    }
    res
}

fn count_calories(data: &Vec<Vec<u32>>) -> u32
{
    data.iter().map(|s| s.iter().sum::<u32>()).max().unwrap()
}

fn count_calories_ordered(data: &Vec<Vec<u32>>) -> Vec<u32>
{
    let mut data = data.iter().map(|s| s.iter().sum::<u32>()).collect::<Vec<u32>>();
    data.sort();
    data.reverse();
    data
}

pub fn door1_1(input: &str) -> u32
{
    let data = parse(input);
    count_calories(&data)
}

pub fn door1_2(input: &str) -> u32
{
    let data = parse(input);
    let cal = count_calories_ordered(&data);
    cal[0] + cal[1] + cal[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        let input = "
1000
2000
3000

4
5
6


7

8

";

        let result = parse(input);

        let expect: Vec<Vec<u32>> = vec![
            vec![ 1000, 2000, 3000 ],
            vec![4, 5, 6],
            vec![7],
            vec![8]
        ];

        assert_eq!(result, expect);
    }

    #[test]
    fn parse_works2() {
        let input = "
1000
 2000
3000

4
5
6


7

8";

        let result = parse(input);

        let expect: Vec<Vec<u32>> = vec![
            vec![ 1000, 2000, 3000 ],
            vec![4, 5, 6],
            vec![7],
            vec![8]
        ];

        assert_eq!(result, expect);
    }

    #[test]
    fn analyze_works()
    {
        let input: Vec<Vec<u32>> = vec![
            vec![ 1000, 2000, 3000 ],
            vec![4, 5, 6],
            vec![7],
            vec![8]
        ];
        let output = count_calories(&input);
        assert_eq!(output, 6000);
    }

    #[test]
    fn door1_1_works()
    {
        let input =
            "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        let output = door1_1(input);
        assert_eq!(output, 24000);
    }

    #[test]
    fn door1_2_works()
    {
        let input =
            "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        let output = door1_2(input);
        let expected = 24000 + 11000 + 10000;
        assert_eq!(output, expected);
    }
}
