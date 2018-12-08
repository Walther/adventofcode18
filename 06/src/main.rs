extern crate rayon;

use rayon::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    closest: Option<Marker>,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
            closest: None,
        }
    }

    pub fn set_closest(&mut self, marker: Marker) {
        self.closest = Some(marker);
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Marker {
    x: i32,
    y: i32,
}

impl Marker {
    pub fn new(x: i32, y: i32) -> Marker {
        Marker { x, y }
    }
}

type Distances = HashMap<Marker, i32>;

type Grid = HashSet<Point>;

type Markers = Vec<Marker>;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut grid = Grid::new();
    let mut markers: Markers = Vec::new();
    // Add all Markers
    for line in INPUT.lines() {
        // TODO: there must be better ways for input parsing
        let coord: Vec<i32> = line.split(", ").map(|item| item.parse().unwrap()).collect();
        //println!("{},{}", coord[0], coord[1]);
        let marker = Marker::new(coord[0], coord[1]);
        markers.push(marker);
    }

    // Figure out boundaries
    let a_marker = markers[0].clone();
    let mut left_top = Point::new(a_marker.x, a_marker.y);

    let mut right_bot = Point::new(a_marker.x, a_marker.y);
    for marker in markers.iter() {
        if marker.x < left_top.x {
            left_top.x = marker.x;
        }
        if marker.y < left_top.y {
            left_top.y = marker.y;
        }
        if marker.x > right_bot.x {
            right_bot.x = marker.y;
        }
        if marker.y > right_bot.y {
            right_bot.y = marker.y;
        }
    }

    // Count distances to all Markers
    // TODO: rayon parallel power
    let mut distance_sums: Vec<i32> = Vec::new();
    for x in left_top.x..=right_bot.x {
        for y in left_top.y..=right_bot.y {
            let mut tile_point = Point::new(x, y);
            let mut distances: Distances = HashMap::new();
            for marker in &markers {
                let distance = (x - marker.x).abs() + (y - marker.y).abs();
                //println!("{}", distance);
                distances.insert(*marker, distance);
            }

            let dist_closest: i32 = *distances.values().min().unwrap();
            let dist_sum: i32 = distances.values().sum::<i32>();
            distance_sums.push(dist_sum);
            // We're only interested in the closest ones, filter rest out
            // FIXME: this is again a bit ugly
            let closest: Vec<(Marker, i32)> = distances
                .into_par_iter()
                .filter(|(_marker, distance)| distance == &dist_closest)
                .collect();
            if closest.len() == 1 {
                tile_point.set_closest(closest[0].0);
            }

            grid.insert(tile_point);
        }
    }

    // Now we know closest Marker for each tile of the Grid
    // Now we need to filter out the markers whose areas touch the edges

    let mut islands = markers.clone();
    for x in left_top.x..=right_bot.x {
        for y in left_top.y..=right_bot.y {
            if x == left_top.x || x == right_bot.x || y == left_top.y || y == right_bot.y {
                islands.retain(|marker| !(marker.x == x && marker.y == y));
            }
        }
    }

    // Sort by size of island
    islands.sort_by(|a, b| area(a, &grid).cmp(&area(b, &grid)));
    let largest_area = area(islands.last().unwrap(), &grid);

    // Part 1
    println!("{}", largest_area);

    // Part 2
    let safe_area = distance_sums.iter().filter(|&dist| dist < &10_000).count();
    println!("{}", safe_area);
}

fn area(marker: &Marker, grid: &Grid) -> i32 {
    let mut area: i32 = 0;
    for tile in grid {
        match tile.closest {
            Some(mark) => {
                if mark.x == marker.x && mark.y == marker.y {
                    area += 1
                }
            }
            None => (),
        }
    }
    area
}
