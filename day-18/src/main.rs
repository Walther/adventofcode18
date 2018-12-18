use crate::Acre::*;
use hashbrown::HashMap;

#[derive(PartialEq, Hash, Clone)]
enum Acre {
    Ground,
    Tree,
    Lumberyard,
}

type LumberArea = HashMap<(i32, i32), Acre>;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut map: LumberArea = HashMap::new();
    // Parse input
    for (y, line) in INPUT.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let acre = match ch {
                '.' => Acre::Ground,
                '|' => Acre::Tree,
                '#' => Acre::Lumberyard,
                _ => Acre::Ground,
            };
            map.insert((x as i32, y as i32), acre);
        }
    }

    let mut history: Vec<LumberArea> = Vec::new();

    let mut minute = 0;
    let mut loop_start = 0;

    while minute <= 1_000_000_000 {
        minute += 1;
        map = step_lumberarea(&map);
        // part 1
        if minute == 10 {
            let value = valuate(&map);
            println!("{}", value);
        }
        // part 2
        if history.contains(&map) {
            loop_start = history.iter().position(|elem| elem == &map).unwrap();
            break;
        }
        history.push(map.clone());
    }
    let loop_end = minute - 1;
    let uniques = loop_end - loop_start;
    let similar_state_index = ((1_000_000_000 - loop_end) % uniques) + loop_start;
    map = history[similar_state_index - 1].clone();
    let value = valuate(&map);
    println!("{}", value);
}

fn valuate(lumber_area: &LumberArea) -> i32 {
    let trees = lumber_area.values().filter(|&acre| *acre == Tree).count() as i32;
    let lumberyards = lumber_area
        .values()
        .filter(|&acre| *acre == Lumberyard)
        .count() as i32;
    trees * lumberyards
}

fn step_lumberarea(lumber_area: &LumberArea) -> LumberArea {
    let mut new_map: LumberArea = HashMap::new();
    for (x, y) in lumber_area.keys() {
        let new_acre = step_acre(*x, *y, lumber_area);
        new_map.insert((*x, *y), new_acre);
    }
    new_map
}

fn step_acre(x: i32, y: i32, lumber_area: &LumberArea) -> Acre {
    let current = safe_get_type(x, y, lumber_area);
    let mut neighbors: Vec<&Acre> = Vec::new();
    for x_diff in -1..=1 {
        for y_diff in -1..=1 {
            // Skip current tile
            if !(x_diff == 0 && y_diff == 0) {
                let acre = safe_get_type(x + x_diff, y + y_diff, lumber_area);
                neighbors.push(acre);
            }
        }
    }
    match current {
        Ground => {
            if neighbors.iter().filter(|&acre| *acre == &Tree).count() >= 3 {
                Tree
            } else {
                Ground
            }
        }
        Tree => {
            if neighbors
                .iter()
                .filter(|&acre| *acre == &Lumberyard)
                .count()
                >= 3
            {
                Lumberyard
            } else {
                Tree
            }
        }
        Lumberyard => {
            if neighbors
                .iter()
                .find(|&acre| *acre == &Lumberyard)
                .is_some()
                && neighbors.iter().find(|&acre| *acre == &Tree).is_some()
            {
                Lumberyard
            } else {
                Ground
            }
        }
    }
}

fn safe_get_type(x: i32, y: i32, lumber_area: &LumberArea) -> &Acre {
    match lumber_area.get(&(x, y)) {
        Some(acre) => &acre,
        None => &Ground,
    }
}
