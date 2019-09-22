use std::collections::HashMap;

#[derive(Debug)]
struct Room {
    north: bool,
    east: bool,
    west: bool,
    south: bool,
    visited: bool,
    distance: i32,
}

impl Room {
    pub fn new() -> Room {
        Room {
            north: false,
            east: false,
            west: false,
            south: false,
            visited: false,
            distance: 0,
        }
    }
}

fn main() {
    let mut map: HashMap<(i32, i32), Room> = HashMap::new();
    // Stack: store the starting positions of branches
    // and the ending positions of those branches
    let mut stack: Vec<((i32, i32), Vec<(i32, i32)>)> = Vec::new();
    let mut coord = (0, 0);
    stack.push(((coord.0, coord.1), Vec::new()));
    // Euclidian coordinate system: start at x 0, y 0; north and east are positive
    const INPUT: &str = include_str!("ENWWW(NEEE|SSE(EE|N)).txt");
    let mut input = INPUT.chars();
    // Skip initial ^
    input.next();
    // Insert all rooms
    for letter in input {
        // Tricky bit: if we've encountered branching already, we need to
        // continue following the rest of the instructions
        // _from all of the previous branch ends_
        let (mut coord, mut branches) = stack.last().unwrap();
        if branches.len() > 0 {
            for branch in branches {
                handle(branch.clone(), letter, &mut map, stack, branches);
            }
        } else {
            handle(coord, letter, &mut map, stack, branches);
        }
    }

    // Crawl initial room manually
    let coord = (0, 0);
    let mut room = map.get_mut(&(coord.0, coord.1)).unwrap();
    room.visited = true;
    // Crawl all rooms
    crawl(coord, &mut map, 0);

    // Debug print
    println!("{:?}", map);

    // Part1: print longest distance
    let longest = map.values().map(|room| room.distance).max().unwrap();
    println!("{}", longest);
}

fn handle(
    coord: (i32, i32),
    letter: char,
    map: &mut HashMap<(i32, i32), Room>,
    stack: Vec<((i32, i32), Vec<(i32, i32)>)>,
    branches: Vec<(i32, i32)>,
) {
    let room = map.entry((coord.0, coord.1)).or_insert(Room::new());
    match letter {
        'N' => {
            room.north = true;
            coord.1 += 1;
        }
        'E' => {
            room.east = true;
            coord.0 += 1;
        }
        'W' => {
            room.west = true;
            coord.0 += -1;
        }
        'S' => {
            room.south = true;
            coord.1 += -1;
        }
        '(' => {
            stack.push((coord, Vec::new()));
        }
        ')' => {
            stack.pop();
            let (new_coord, new_branches) = stack.last().unwrap();
            branches = *new_branches;
            coord = *new_coord;
        }
        '|' => {
            let current = coord;
            let (mut new_coord, mut new_branches) = stack.last().unwrap();
            branches = new_branches;
            branches.push(current);
            coord = new_coord;
        }
        '$' => {
            return;
        }
        _ => unreachable!(),
    }
}

fn crawl(coord: (i32, i32), map: &mut HashMap<(i32, i32), Room>, distance: i32) {
    let room = map.get_mut(&(coord.0, coord.1)).unwrap();
    // BFS flood-fill for visits & distance
    // We only update distance on the first visit, as that's the shortest path
    let mut neighbors: Vec<(i32, i32)> = Vec::new();
    if room.north {
        neighbors.push((coord.0, coord.1 + 1));
    }
    if room.east {
        neighbors.push((coord.0 + 1, coord.1));
    }
    if room.west {
        neighbors.push((coord.0 - 1, coord.1 + 1));
    }
    if room.south {
        neighbors.push((coord.0, coord.1 - 1));
    }
    let dist = distance + 1;
    for neighbor_coord in neighbors.iter() {
        // Mark the room
        let mut neighbor = map.get_mut(&(neighbor_coord.0, neighbor_coord.1)).unwrap();
        if neighbor.visited {
            return;
        } else {
            neighbor.visited = true;
            neighbor.distance = dist;
        }
        // Recurse
        crawl(*neighbor_coord, map, dist);
    }
}
