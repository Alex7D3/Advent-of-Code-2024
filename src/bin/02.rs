use advent_of_code::common;

fn main() {
    let puzzle = common::read_input(2);
    let output1 = part_one(&puzzle);
    println!("part_one {}", output1);
}

fn parse() {}

fn part_one(puzzle: &str) -> usize {
    let mut safe_count = 0;
    puzzle
        .lines()
        .filter(|&row| {
            let report: Vec<i32> = row
                .split_whitespace()
                .filter_map(|lvl| lvl.parse().ok())
                .collect();

            is_safe(&report)
        })
        .count()
}

fn part_two(puzzle: &str) -> usize {
    puzzle
        .lines()
        .filter(|&row| {
            let report: Vec<i32> = row
                .split_whitespace()
                .filter_map(|lvl| lvl.parse().ok())
                .collect();

            if (!is_safe(&report)) {
                false
            } else {
                true
            }
        })
        .count()
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        true
    } else {
        let order = report[0].cmp(&report[1]);
        report.windows(2).all(|win| {
            let dif = (win[0] - win[1]).abs();
            order != win[0].cmp(&win[1]) || 1 <= dif && dif <= 3
        })
    }
}

fn is_safe_dampen(report: &[i32]) -> bool {}

#[cfg(test)]
mod tests {
    // use super::*;
}
