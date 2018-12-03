use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    // 2d vector of HashSets: each square stores the ID's that use that square
    let mut fabric: HashMap<(usize, usize), HashSet<usize>> = HashMap::new();
    let mut ids: HashSet<usize> = HashSet::new();

    for line in INPUT.lines() {
        // Parse the input
        let parts: Vec<usize> = line
            .split(|c| " @#:,x".chars().any(|d| c == d))
            .filter(|s| !s.is_empty())
            .map(|part| part.parse().unwrap())
            .collect();

        let id: usize = parts[0];
        ids.insert(id);

        let start_x: usize = parts[1];
        let start_y: usize = parts[2];

        let size_x: usize = parts[3];
        let size_y: usize = parts[4];

        // For each square in the defined area, insert ID in fabric
        for x in start_x..start_x + size_x {
            for y in start_y..start_y + size_y {
                fabric.entry((x, y)).or_insert(HashSet::new()).insert(id);
            }
        }
    }

    // Count the squares that are used more than once
    // Remove overlapping ids from ids list
    let mut overlaps = 0;
    for (_, square) in fabric {
        if square.len() > 1 {
            overlaps += 1;
            for id in square {
                ids.remove(&id);
            }
        }
    }

    // Part 1
    println!("{}", overlaps);

    // Part 2
    println!("{:?}", ids)
}
