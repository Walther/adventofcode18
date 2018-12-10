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
        let a = regex.captures(line).unwrap()[1].to_string();
        tasks.entry(a.clone()).or_insert(Vec::new());
        let b = regex.captures(line).unwrap()[2].to_string();
        // B requires: A
        tasks.entry(b).or_insert(Vec::new()).push(a.clone());
    }
    //println!("{:?}", tasks);

    // Part 1
    let tasks1 = tasks.clone();
    let order1 = do_all(&tasks1, Vec::new());
    let order1: String = order1.iter().fold(String::new(), |mut acc, string| {
        acc.push_str(string);
        return acc;
    });
    println!("{}", order1);

    // Part 2
    // TODO: clean up this mess. lots of repeating myself / spaghetti code
    let mut tasks2 = tasks.clone();
    // generated the list - can't iterate over a Range of Chars
    let letters = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
    .iter()
    .map(|c| c.to_string());
    let len = letters.len() as i32;
    let completion_times: HashMap<String, i32> = letters.zip(61..=(61 + len)).collect();
    let mut time = 0;
    let mut tasks_in_progress: HashMap<String, i32> = HashMap::new();
    let workers = 5;
    let mut order2: Vec<String> = Vec::new();
    loop {
        // Do the work
        for (task, time_left) in tasks_in_progress.iter_mut() {
            *time_left -= 1;
            // If task got finished, add it to completion order
            if *time_left == 0 {
                order2.push(task.clone());
                // And remove dependencies
                for (_task, deps) in tasks2.iter_mut() {
                    deps.retain(|dep| dep != task);
                }
            }
        }
        // Remove done entries (can't remove in above loop due to borrowing?)
        tasks_in_progress.retain(|_task, time_left| time_left > &mut 0); // Wat, why does this zero have to be mut
                                                                         //println!("{:?}", tasks_in_progress);

        // If we don't have enough tasks running, add work
        while tasks_in_progress.len() < workers {
            let doable: Tasks = tasks2
                .iter()
                .filter(|(_task, deps)| deps.len() == 0)
                .map(|(a, b)| (a.to_string(), b.clone()))
                .collect();
            if doable.len() == 0 {
                // Add only if we can
                break;
            }
            let do_now = doable.keys().min().unwrap();
            *tasks_in_progress.entry(do_now.clone()).or_insert(0) =
                *completion_times.get(do_now).unwrap();
            tasks2.remove(do_now);
        }
        if tasks_in_progress.len() == 0 {
            // All done
            break;
        }
        time += 1;
    }
    /* let order2: String = order2.iter().fold(String::new(), |mut acc, string| {
        acc.push_str(string);
        return acc;
    });
    println!("{:?}", order2);
    */
    println!("{}", time);
}

fn step(tasks: &Tasks) -> (String, Tasks) {
    let mut remaining_tasks = tasks.clone();
    let doable: Tasks = remaining_tasks
        .iter()
        .filter(|(_task, deps)| deps.len() == 0)
        .map(|(a, b)| (a.to_string(), b.clone()))
        .collect();
    //println!("{:?}", doable);

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
    //println!("{}", task);
    order.push(task);
    if remaining.len() == 0 {
        return order;
    } else {
        return do_all(&remaining, order);
    }
}
