use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn adjacent(&self, p: &Point) -> bool
    {
        i32::abs(self.x - p.x) <= 1 && i32::abs(self.y - p.y) <= 1
    }

    fn move_towards(&mut self, p: &Point)
    {
        //println!("{:?} {:?}", self, p);
        match self.x - p.x
        {
            2|1 => self.x -= 1,
            -2|-1 => self.x += 1,
            0 => (),
            _ => panic!("Error")
        }

        match self.y - p.y
        {
            2|1 => self.y -= 1,
            -2|-1 => self.y += 1,
            0 => (),
            _ => panic!("Error")
        }

    }
}

fn simulate_1_step(points: &mut Vec<Point>, history: &mut HashSet<Point>, dir: char)
{
    let mut points = points.iter_mut();

    let head = points.next().unwrap();

    match dir {
        'U' => head.y += 1,
        'D' => head.y -= 1,
        'L' => head.x -= 1,
        'R' => head.x += 1,
        _ => assert!(false)
    }

    let mut prev = head.clone();
    for p in points {
        if p.adjacent(&prev)
        {
            prev = p.clone();
            continue;
        }

        /* Move */
        p.move_towards(&prev);

        prev = p.clone();
    }

    history.insert(prev);
}

fn parse_line(line: &str) -> (char, u8)
{
    let mut chars = line.chars();

    let dir = chars.next().unwrap();
    let steps = chars.skip(1).collect::<String>().parse::<u8>().unwrap();

    (dir, steps)
}

pub fn door9_part_1(input: &str) -> u32
{
    let tail = Point {
        x: 0,
        y: 0
    };
    let head = Point {
        x: 0,
        y: 0
    };

    let mut history: HashSet<Point> = HashSet::new();
    history.insert(tail.clone());

    let mut rope = vec![head, tail];

    for line in input.lines()
    {
        let (dir, steps) = parse_line(line);
        for _ in 0..steps {
            simulate_1_step(&mut rope, &mut history, dir)
        }
    }

    history.len() as u32
}

pub fn door9_part_2(input: &str) -> u32
{
    let p = Point {
        x: 0,
        y: 0
    };

    let mut history: HashSet<Point> = HashSet::new();
    history.insert(p.clone());

    let mut rope = vec![];

    for _ in 0..10
    {
        rope.push(p.clone());
    }

    for line in input.lines()
    {
        let (dir, steps) = parse_line(line);
        for _ in 0..steps {
            simulate_1_step(&mut rope, &mut history, dir)
        }
    }

    history.len() as u32
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parsing_works() {
        assert_eq!(parse_line("U 5"), ('U', 5));
        assert_eq!(parse_line("L 15"), ('L', 15));
    }

    #[test]
    fn part_1_works() {
        let input = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
".trim();
        assert_eq!(door9_part_1(input), 13);
    }

    #[test]
    fn part_2_works_1() {
        let input = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
".trim();
        assert_eq!(door9_part_2(input), 1);
    }

    #[test]
    fn part_2_works_2() {
        let input = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
".trim();
        assert_eq!(door9_part_2(input), 36);
    }
}
