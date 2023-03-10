use std::io;
use std::fs;

use door1;
use door2;
use door3;
use door4;
use door5;
use door6;
use door8;
use door9;

fn get_input(stdin: &io::Stdin) -> String
{
    println!("Input (end with !)");
    let mut input = String::new();

    loop {
        let mut line = String::new();

        stdin.read_line(&mut line).unwrap();

        if line.trim() == "!"
        {
            break;
        }

        input.push_str(&line);
    }
    input
}

fn load_input_from_file(file: &str) -> String
{
    fs::read_to_string(file).unwrap()
}

fn main() {
    let stdin = io::stdin();

    println!("Hello traveller, which puzzle are we cracking today?");
    let mut puzzle = String::new();
    stdin.read_line(&mut puzzle).unwrap();
    println!("{}", puzzle);
    let puzzle : u32 = puzzle.trim().parse::<u32>().unwrap();

    println!("Which part?");
    let mut part = String::new();
    stdin.read_line(&mut part).unwrap();
    let part : u32 = part.trim().parse::<u32>().unwrap();

    println!("Ok, let's see what puzzle part {} of puzzle {} brings us.", part, puzzle);

    match (puzzle, part)
    {
        (1, 1) => {
            let input = get_input(&stdin);
            let result = door1::door1_1(&input);
            println!("Solution is {}", result);
        }
        (1, 2) => {
            let input = get_input(&stdin);
            let result = door1::door1_2(&input);
            println!("Solution is {}", result);
        }
        (2, 1) => {
            let input = get_input(&stdin);
            let result = door2::door_2_1(&input);
            println!("Solution is {}", result);
        }
        (2, 2) => {
            let input = get_input(&stdin);
            let result = door2::door_2_2(&input);
            println!("Solution is {}", result);
        }
        (3, 1) => {
            let input = get_input(&stdin);
            let result = door3::door_3_1(&input);
            println!("Solution is {}", result);
        }
        (3, 2) => {
            let input = get_input(&stdin);
            let result = door3::door_3_2(&input);
            println!("Solution is {}", result);
        }
        (4, 1) => {
            let input = get_input(&stdin);
            let result = door4::door_4_1(&input);
            println!("Solution is {}", result);
        }
        (4, 2) => {
            let input = get_input(&stdin);
            let result = door4::door_4_2(&input);
            println!("Solution is {}", result);
        }
        (5, 1) => {
            let input = get_input(&stdin);
            let result = door5::door_5_1(&input);
            println!("Solution is {}", result);
        }
        (5, 2) => {
            let input = get_input(&stdin);
            let result = door5::door_5_2(&input);
            println!("Solution is {}", result);
        }
        (6, 1) => {
            let input = load_input_from_file("../door6/input.txt");
            let result = door6::door_6_1(&input);
            println!("Solution is {}", result);
        }
        (6, 2) => {
            let input = load_input_from_file("../door6/input.txt");
            let result = door6::door_6_2(&input);
            println!("Solution is {}", result);
        }
        (8, 1) => {
            let input = load_input_from_file("../door8/input.txt");
            let result = door8::door_8_1(&input);
            println!("Solution is {}", result);
        }
        (8, 2) => {
            let input = load_input_from_file("../door8/input.txt");
            let result = door8::door_8_2(&input);
            println!("Solution is {}", result);
        }
        (9, 1) => {
            let input = load_input_from_file("../door9/input.txt");
            let result = door9::door9_part_1(&input);
            println!("Solution is {}", result);
        }
        (9, 2) => {
            let input = load_input_from_file("../door9/input.txt");
            let result = door9::door9_part_2(&input);
            println!("Solution is {}", result);
        }
        (_, _) => {
            println!("Oh no, 404, abort. This riddle is not solved, yet.");
        }
    }
}
