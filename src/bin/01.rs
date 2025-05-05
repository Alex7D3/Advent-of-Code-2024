use advent_of_code::common;
use std::{self, collections::HashMap};
fn main() {
    let input = common::read_input(1);
    let (list1, list2) = parse_input(&input);
    let output1 = part_one(list1.clone(), list2.clone());
    let output2 = part_two(&list1, &list2);
    println!("part_one: {output1}");
    println!("part_two: {output2}");
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        list1.push(iter.next().unwrap().parse::<i32>().unwrap());
        list2.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    (list1, list2)
}

fn part_one(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();
    std::iter::zip(list1, list2)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part_two(list1: &[i32], list2: &[i32]) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &id in list2 {
        *counts.entry(id).or_insert(0) += 1;
    }

    list1
        .iter()
        .map(|id| *id * counts.get(id).copied().unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    // use super::*;
}
