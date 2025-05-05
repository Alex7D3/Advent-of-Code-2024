use regex::Regex;
use advent_of_code::common;

fn main() {
    let puzzle = common::read_input(3);
    let output1 = part_one(&puzzle);
    let output2 = part_two(&puzzle);
    println!("part_one: {}, part_two: {}", output1, output2);
}

fn part_one(puzzle: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(puzzle).map(|c| {
        let (_, [arg1, arg2]) = c.extract();
        arg1.parse::<i32>().unwrap() * arg2.parse::<i32>().unwrap()
    }).sum()
}

fn part_two(puzzle: &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut enable_flag = true;
    re.find_iter(puzzle).filter_map(|m| {
        let instruction = m.as_str();
        match instruction {
            "do()" => {
                enable_flag = true;
                None
            },
            "don't()" => {
                enable_flag = false;
                None
            },
            _ => if enable_flag {
                Some(instruction[4..instruction.len()-1].split(',').map(|arg: &str|
                    arg.trim().parse::<i32>().unwrap()
                )
                .fold(1, |acc, arg| {println!("{}",arg); acc * arg}))
            } else {
                None
            }
        }
    })
    .sum()
}
