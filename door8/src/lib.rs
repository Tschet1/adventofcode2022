use std::collections::HashSet;

struct Map {
    state: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn add_line(&mut self, line: &str)
    {
        self.height +=1;
        self.state.extend(line.chars().map(|c| c as u8 - 48));
    }

    fn create_from_line(line: &str) -> Map
    {
        let mut map: Map = Map{
            state: vec![],
            width: 0,
            height: 0,
        };
        map.add_line(line);
        map.width = map.state.len();
        map.height = 1;
        map
    }

    fn get(&self, x: usize, y: usize) -> u8
    {
        self.state[x*self.width + y]
    }

    fn iter_row(&self, x: usize) -> impl Iterator<Item = &u8>
    {
        self.state.iter().skip(x*self.width).enumerate().map_while(|(c, x)| {
            if c < self.width {
                Some(x)
            }
            else {
                None
            }
        })
    }

    fn iter_col(&self, y: usize) -> impl Iterator<Item = &u8>
    {
        self.state.iter().skip(y).step_by(self.width)
    }

    fn iter_row_rev(&self, x: usize) -> impl Iterator<Item = &u8>
    {
        self.state.iter().rev().skip((self.height - x - 1)*self.width).enumerate().map_while(|(c, x)| {
            if c < self.width {
                Some(x)
            }
            else {
                None
            }
        })
    }

    fn iter_col_rev(&self, y: usize) -> impl Iterator<Item = &u8>
    {
        self.state.iter().rev().skip(self.width - y - 1).step_by(self.width)
    }

}

fn create_map(lines: &str) -> Map
{
    let mut lines = lines.lines();
    let mut map = Map::create_from_line(lines.next().unwrap());

    for line in lines
    {
        map.add_line(line);
    }
    map
}

fn print_marked(map: &HashSet<(usize, usize)>, width: usize, height: usize)
{
    for x in 0..height {
        for y in 0..width {
            if map.contains(&(x,y))
            {
                print!("x")
            }
            else
            {
                print!(" ");
            }
        }
        print!("\n");
    }
}

pub fn door_8_1(lines: &str) -> usize
{
    let map = create_map(lines);
    let mut marked: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..map.width {
        let mut highest : i16 = -1;
        for (x, e) in map.iter_col(y).enumerate() {
            if *e as i16> highest {
                highest = *e as i16;
                marked.insert((x, y));

                if highest == 9
                {
                    break;
                }
            }
        }

        highest = -1;
        for (x, e) in map.iter_col_rev(y).enumerate() {
            if *e as i16 > highest {
                highest = *e as i16;
                marked.insert((map.height - x - 1, y));

                if highest == 9
                {
                    break;
                }
            }
        }
    }

    for x in 0..map.height {
        let mut highest : i16 = -1;
        for (y, e) in map.iter_row(x).enumerate() {
            if *e as i16 > highest {
                highest = *e as i16;
                marked.insert((x, y));

                if highest == 9
                {
                    break;
                }
            }
        }

        highest = -1;
        for (y, e) in map.iter_row_rev(x).enumerate() {
            if *e as i16 > highest {
                highest = *e as i16;
                marked.insert((x, map.width - y - 1));

                if highest == 9
                {
                    break;
                }
            }
        }
    }

    //print_marked(&marked, map.width, map.height);
    return marked.len();
}

fn get_line_length<'a>(iter: impl Iterator<Item = &'a u8>, init_val: u8) -> u32
{
    let mut view = 0;

    let mut highest = init_val;
    for e in iter {
        view += 1;

        if highest <= *e
        {
            break;
        }
    }

    view
}

pub fn door_8_2(lines: &str) -> u32
{
    let map = create_map(lines);

    let mut high_score = 0;

    for x_init in 0..map.height {
        for y_init in 0..map.width {
            let highest_const : u8 = map.get(x_init, y_init);
            let mut score = 1;

            score *= get_line_length(map.iter_col(y_init).skip(x_init + 1), highest_const);
            score *= get_line_length(map.iter_row(x_init).skip(y_init + 1), highest_const);
            score *= get_line_length(map.iter_col_rev(y_init).skip(map.height - x_init), highest_const);
            score *= get_line_length(map.iter_row_rev(x_init).skip(map.width - y_init), highest_const);

            if score > high_score
            {
                high_score = score;
            }
        }
    }

    high_score
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        let input = "
30373
25512
65332
33549
35390".trim();

        let map = create_map(input);
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 5);
        assert_eq!(map.get(4,3), 9);
        assert_eq!(map.iter_col(0).collect::<Vec<&u8>>(), vec![&3, &2, &6, &3, &3]);
        assert_eq!(map.iter_row(0).collect::<Vec<&u8>>(), vec![&3, &0, &3, &7, &3]);
        assert_eq!(map.iter_col(4).collect::<Vec<&u8>>(), vec![&3, &2, &2, &9, &0]);
        assert_eq!(map.iter_row(4).collect::<Vec<&u8>>(), vec![&3, &5, &3, &9, &0]);
        assert_eq!(map.iter_col_rev(4).collect::<Vec<&u8>>(), vec![&0, &9, &2, &2, &3]);
        assert_eq!(map.iter_row_rev(4).collect::<Vec<&u8>>(), vec![&0, &9, &3, &5, &3]);
        assert_eq!(map.iter_row_rev(0).collect::<Vec<&u8>>(), vec![&3, &7, &3, &0, &3]);
    }

    #[test]
    fn part1_works() {
        let input = "
30373
25512
65332
33549
35390".trim();
        assert_eq!(door_8_1(input), 21);
    }

    #[test]
    fn part1_works_2() {
        let input = "
30373
25512
65332
33549
35390
00000".trim();
        assert_eq!(door_8_1(input), 26);
    }

    #[test]
    fn part2_works() {
        let input = "
30373
25512
65332
33549
35390".trim();
        assert_eq!(door_8_2(input), 8);
    }

    #[test]
    fn part2_works_2() {
        let input = "
98765
88888
77777
66666
55555".trim();
        assert_eq!(door_8_2(input), 3);
    }

    #[test]
    fn part2_works_3() {
        let input = "
11111
22222
33433
33333
22522
11111".trim();
        assert_eq!(door_8_2(input), 16);
    }


    #[test]
    fn part2_works_4() {
        let input = "
11111
22222
33433
33333
22252
11111".trim();
        assert_eq!(door_8_2(input), 24);
    }

    #[test]
    fn part2_works_5() {
        let input = "
11111
22622
33433
33333
22252
11111".trim();
        assert_eq!(door_8_2(input), 16);
    }

    #[test]
    fn part2_works_6() {
        let input = "
11111
22422
34443
33433
22242
11111".trim();
        assert_eq!(door_8_2(input), 8);
    }

    #[test]
    fn part2_works_7() {
        let input = "
11111
22222
33133
33333
22252
11111".trim();
        assert_eq!(door_8_2(input), 12);
    }
}
