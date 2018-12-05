extern crate rayon;
use rayon::prelude::*;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let input: Vec<char> = INPUT.chars().collect();

    // part 1
    let polymer = polymerize(&input);
    println!("{}", polymer.len());

    // part 2
    // generated the list - can't iterate over a Range of Chars
    let units = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let polymers: Vec<Vec<char>> = units
        .par_iter() // Parallel power!
        .map(|unit| polymerize(&filter_unit(&input, *unit)))
        .collect();

    let shortest = polymers
        .par_iter()
        .map(|polymer| polymer.len())
        .min()
        .unwrap();
    println!("{}", shortest)
}

fn polymerize(input: &Vec<char>) -> Vec<char> {
    // Poor man's stack implementation
    let mut polymer: Vec<char> = Vec::new();
    let mut last: char = ' ';
    for letter in input {
        if letter.is_lowercase() != last.is_lowercase()
            && letter.to_lowercase().to_string() == last.to_lowercase().to_string()
        {
            polymer.pop();
            last = match polymer.last() {
                Some(c) => *c,
                None => ' ',
            };
            continue;
        } else {
            polymer.push(*letter);
            last = *letter;
        }
    }
    polymer
}

fn filter_unit(input: &Vec<char>, remove: char) -> Vec<char> {
    input
        .iter()
        .filter(|letter| letter.to_lowercase().to_string() != remove.to_string())
        .cloned()
        .collect()
}
