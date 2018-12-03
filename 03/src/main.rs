use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    // 2d vector of HashSets: each square stores the ID's that use that square
    let mut fabric: HashMap<(usize, usize), HashSet<usize>> = HashMap::new();
    let mut ids: HashSet<usize> = HashSet::new();

    for line in INPUT.lines() {
        let instruction: Vec<&str> = line.split(" ").collect();
        //println!("{:?}", instruction);
        let id: usize = instruction[0].get(1..).unwrap().parse().unwrap();
        ids.insert(id);
        //println!("{}", id);
        let coords: Vec<&str> = instruction[2].split(",").collect();
        let start_x: usize = coords[0].parse().unwrap();
        let start_y: usize = coords[1]
            .get(..coords[1].len() - 1)
            .unwrap()
            .parse()
            .unwrap();
        //println!("{}", start_x);
        //println!("{}", start_y);
        let size: Vec<&str> = instruction[3].split("x").collect();
        let size_x: usize = size[0].parse().unwrap();
        let size_y: usize = size[1].parse().unwrap();

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
