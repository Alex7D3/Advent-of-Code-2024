
use advent_of_code::common;

fn main() {
    let input = common::read_input(4);
    let puzzle = parse(&input);
    let output1 = part_one(&puzzle);
    let output2 = part_two(&puzzle);
    println!("part_one: {}, part_two: {}", output1, output2);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line|
        line.chars().collect())
    .collect()
}

fn part_one(grid: &Vec<Vec<char>>) -> usize {
    let (m, n) = (grid.len(), grid[0].len());
    let mut count: usize = 0;
    
    let dirs = [
        (0, 1), (1, 0), (0, -1), (-1, 0), (-1, 1), (1, 1), (1, -1), (-1, -1)
    ];

    for i in 0..m {
        for j in 0..n {
            count += dirs.iter().filter(|dir|
                grid[i][j] == 'X' && find_mas(i as i32, j as i32, dir, grid)
            ).count();  
        }
    }
    count
}

fn part_two(grid: &Vec<Vec<char>>) -> usize {
    let (m, n) = (grid.len(), grid[0].len());
    let dirs = [
        (-1, 1), (1, 1), (1, -1), (-1, -1)
    ];
    let mut count: usize = 0;
    for i in 0..m {
        for j in 0..n {
            if dirs.iter().filter(|dir|
                grid[i][j] == 'A' &&
                find_mas(
                    i as i32 + dir.0 * -2,
                    j as i32 + dir.1 * -2, 
                    dir, grid
                )
            ).count() == 2 {
                count += 1;
            } 
        }
    }
    count
}

fn find_mas(mut i: i32, mut j: i32, dir: &(i32, i32), grid: &Vec<Vec<char>>) -> bool {
    "MAS".chars().all(|c| {
        i += dir.0;
        j += dir.1;
        0 <= i && i < grid.len() as i32 &&
        0 <= j && j < grid[0].len() as i32 &&
        c == grid[i as usize][j as usize]
    })
}


