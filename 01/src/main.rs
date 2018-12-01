use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");

    // Parse entire input into a list
    let mut frequencies: Vec<i32> = Vec::new();
    for line in INPUT.lines() {
        let parsed = line.parse::<i32>().unwrap();
        frequencies.push(parsed);
    }

    // Part 1: evaluate final frequency
    let sum: i32 = frequencies.iter().sum();
    println!("{}", sum);

    // Part 2: loop instructions and find first repeat
    let mut frequency = 0;
    let mut index = 0;
    let mut history = HashSet::new();
    history.insert(frequency); // include initial zero frequency

    loop {
        frequency += frequencies[index];

        if history.contains(&frequency) {
            println!("{}", frequency);
            break;
        }

        history.insert(frequency);
        index += 1;
        if index == frequencies.len() {
            index = 0;
        }
    }
}
