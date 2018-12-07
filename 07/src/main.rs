extern crate regex;
use regex::Regex;
use std::collections::HashMap;

type Requirements = Vec<String>;

type Tasks = HashMap<String, Requirements>;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut tasks: Tasks = HashMap::new();

    for line in INPUT.lines() {
        let regex = Regex::new(r"Step ([A-Z]) .* ([A-Z]) .*").unwrap();
        // A must be finished before B
        let A = regex.captures(line).unwrap()[1].to_string();
        tasks.entry(A.clone()).or_insert(Vec::new());
        let B = regex.captures(line).unwrap()[2].to_string();
        // B requires: A
        tasks.entry(B).or_insert(Vec::new()).push(A.clone());
    }
    //println!("{:?}", tasks);

    // Part 1
    let order = do_all(&tasks, Vec::new());
    let order: String = order.iter().fold(String::new(), |mut acc, string| {
        acc.push_str(string);
        return acc;
    });
    println!("{:?}", order);
}

fn step(tasks: &Tasks) -> (String, Tasks) {
    let mut remaining_tasks = tasks.clone();
    let doable: Tasks = remaining_tasks
        .iter()
        .filter(|(_task, deps)| deps.len() == 0)
        .map(|(a, b)| (a.to_string(), b.clone()))
        .collect();
    println!("{:?}", doable);

    if doable.len() == 0 {
        return ("".to_string(), HashMap::new());
    }
    let do_now = doable.keys().min().unwrap();
    remaining_tasks.remove(do_now);
    let done = do_now;
    for (_task, deps) in remaining_tasks.iter_mut() {
        deps.retain(|dep| dep != done);
    }

    (done.to_string(), remaining_tasks)
}

fn do_all(tasks: &Tasks, mut order: Vec<String>) -> Vec<String> {
    //let mut order: Vec<String> = Vec::new();

    let (task, remaining) = step(&tasks);
    println!("{}", task);
    order.push(task);
    if remaining.len() == 0 {
        return order;
    } else {
        return do_all(&remaining, order);
    }
}
