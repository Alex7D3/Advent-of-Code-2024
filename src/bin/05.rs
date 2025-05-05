use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use advent_of_code::common;

type RuleBook = HashMap<i32, HashSet<i32>>;
struct Puzzle {
    rules: RuleBook,
    updates: Vec<Vec<i32>>
}

fn main() {
    let input = common::read_input(5);
    let puzzle: Puzzle = parse(&input);
    let part1 = part_one(&puzzle);
    let part2 = part_two(&puzzle);
    println!("part1: {}, part2: {}", part1, part2);
}

fn parse(input: &str) -> Puzzle {
    let mut rules: RuleBook = HashMap::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let rule: Vec<i32> = line.split('|')
            .filter_map(|page| page.parse().ok())
            .collect();
        rules.entry(rule[0]).and_modify(|neighbors| {
            neighbors.insert(rule[1]);
        })
        .or_insert(HashSet::from_iter([rule[1]]));
    }

    let updates: Vec<Vec<i32>> = lines.map(|update| 
        update.split(',')
        .filter_map(|page| page.parse().ok())
        .collect()
    ).collect();

    Puzzle { rules, updates }
}

fn part_one(puzzle: &Puzzle) -> i32 {
    puzzle.updates.iter().filter_map(|pages| {
        let n = pages.len();
        if is_ordered(&puzzle.rules, &pages) {
            Some(pages[n / 2])
        } else {
            None
        }   
    })
    .sum()
}

fn part_two(puzzle: &Puzzle) -> i32 {
    puzzle.updates.iter().filter(|pages|
        !is_ordered(&puzzle.rules, &pages)  
    )
    .map(|pages| {
        let mut ordered_pages = pages.clone();
        ordered_pages.sort_by(|a, b|
            if puzzle.rules.get(a).is_some_and(|links| links.contains(b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        );
        ordered_pages[ordered_pages.len() / 2]
    })
    .sum()
}


fn is_ordered(rules: &RuleBook, pages: &Vec<i32>) -> bool {
    let n = pages.len();
    for i in 0..n {
        for j in i+1..n {
            if rules.get(&pages[i]).is_none_or(|links| !links.contains(&pages[j])
            ) {
                return false;
            }
        }
    }
    true
}