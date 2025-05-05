use std::collections::HashSet;
use advent_of_code::common;

fn main() {
    let input = common::read_input(6);

    let mut grid: Vec<Vec<char>> = parse(&input);
    let positions = get_positions(&grid);
    let part1 = part_one(&positions);
    let part2 = part_two(&positions, &mut grid);
    println!("part1: {part1}");
    println!("part2: {part2}");
}

#[derive(Clone)]
struct State(i32,i32,Dir);

#[derive(Copy,Clone)]
enum Dir {
    RIGHT,
    DOWN,
    LEFT,
    UP
}

fn part_one(positions: &Vec<State>) -> usize {
    let mut seen_state: HashSet<Cell> = HashSet::new();
    positions.iter().filter(|&pos| {
        if seen_state.contains(&(pos.0, pos.1)) {
            false
        } else {
            seen_state.insert((pos.0, pos.1));
            true
        }
    }).count()
}

fn part_two(positions: &Vec<State>, grid: &mut Vec<Vec<char>>) -> usize {
    positions.iter().skip(1).filter(|&&pos| {
        let temp = grid[pos.0 as usize][pos.1 as usize];
        grid[pos.0 as usize][pos.1 as usize] = '#';
        let path = get_positions(grid);
        grid[pos.0 as usize][pos.1 as usize] = temp;
        
        path.iter().cloned().collect::<HashSet<_>>().len() > 1
    }).count()
    
}


fn get_positions(grid: &Vec<Vec<char>>) -> Vec<State> {
    let n = grid.len() as i32;
    let mut state = find_guard(&grid);
    let mut output = vec![state];
    let mut dir = (-1, 0);
    loop {
        state.0 += dir.0;
        state.1 += dir.1;
        
        println!("{} {}", state.0, state.1);
        if state.0 < 0 || state.0 >= n || state.1 < 0 && state.1 >= n {
            break;
        }
        if grid[state.0 as usize][state.1 as usize] == '#' {
            state.0 -= dir.0;
            state.1 -= dir.1;
            rotate_dir(&mut dir);
        } else {
            output.push(state.clone());
        }
    }
    print!("{:?}\n\n", output);
    output
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line|
        line.chars().collect()
    ).collect()
}

fn rotate_dir(dir: &mut (i32, i32)) {
    let next_dir = 
    *dir = (dir.1, -dir.0);
}

fn find_guard(grid: &Vec<Vec<char>>) -> State {
    grid.iter().enumerate().find_map(|(row_idx, row)|
        row.iter().enumerate().find_map(|(col_idx, &c)| {
            if c == '^' {
                Some(State(row_idx as i32, col_idx as i32, Dir::UP))
            } else {
                None
            }
        })
    )
    .expect("Expected a character: <, >, ^, v")
}