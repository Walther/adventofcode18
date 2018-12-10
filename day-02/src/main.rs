use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("input.txt");

    // Part 1: checksum
    let mut counts: Vec<HashMap<char, i32>> = Vec::new();
    let mut total_twos = 0;
    let mut total_threes = 0;
    let mut index = 0;

    for line in INPUT.lines() {
        counts.push(HashMap::new());
        for letter in line.chars() {
            *counts[index].entry(letter).or_insert(0) += 1;
        }
        // count how many characters appear exactly twice
        let twos = counts[index]
            .iter()
            .filter(|(_, &value)| value == 2)
            .count();
        // count how many characters appear exactly thrice
        let threes = counts[index]
            .iter()
            .filter(|(_, &value)| value == 3)
            .count();
        if twos > 0 {
            total_twos += 1
        }
        if threes > 0 {
            total_threes += 1
        }
        index += 1;
    }
    let checksum = total_twos * total_threes;
    println!("{}", checksum);

    // Part 2: longest common-letter string from all string pairs
    let mut pair_index = 0;
    let mut commons: Vec<String> = Vec::new();
    for line1 in INPUT.lines() {
        for line2 in INPUT.lines() {
            commons.push(String::new());

            if line1 == line2 {
                continue;
            }
            for (letter_index, letter) in line1.chars().enumerate() {
                if letter == line2.chars().nth(letter_index).unwrap() {
                    commons[pair_index].push(letter);
                }
            }
            pair_index += 1;
        }
    }
    commons.sort_by(|a, b| a.len().cmp(&b.len()));
    let solution = commons.last().unwrap();
    println!("{}", solution);
}
