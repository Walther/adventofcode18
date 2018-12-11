use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let grid_serial: i32 = INPUT.parse().unwrap();
    let cells: HashMap<(i32, i32), i32> =
    // Populate the fuel cells
    (1..301).into_par_iter().flat_map(|x| {
        (1..301).into_par_iter().map(move |y| {
            let cell_rack_id = x + 10;
            let mut power_level = cell_rack_id * y;
            power_level = power_level + grid_serial;
            power_level *= cell_rack_id;
            if power_level < 100 {
                power_level = 0;
            } else {
                power_level = (power_level / 100) % 10;
            }
            power_level -= 5;

            ((x, y), power_level)
        })
    }).collect();

    // Part 1
    // Sum all 3x3's && find best
    let sum_cells3 = sum_cells(&cells, 3);

    // Find best
    let ((x, y, _), sum) = sum_cells3
        .into_par_iter()
        .max_by_key(|&((_x, _y, _z), sum)| sum)
        .unwrap();
    println!("{}, {}, {}", x, y, sum);

    // Part 2
    // Get all sums from 1x1 to 300x300
    let sum_cells_all: Vec<HashMap<(i32, i32, i32), i32>> = (1..301)
        .into_par_iter()
        .map(|square_size| sum_cells(&cells, square_size))
        .collect();
    // Find best
    let ((x, y, z), sum) = sum_cells_all
        .iter()
        .map(|hashmap| {
            hashmap
                .iter()
                .max_by_key(|&((_x, _y, _z), sum)| sum)
                .unwrap()
        })
        .max_by_key(|&((_x, _y, _z), sum)| sum)
        .unwrap();
    println!("{}, {}, {}, {}", x, y, z, sum);
}

fn sum_cells(cells: &HashMap<(i32, i32), i32>, square_size: i32) -> HashMap<(i32, i32, i32), i32> {
    println!("Summing window size: {}", square_size);
    // max x coordinate, considering square_size
    let max_x: i32 = 300 - square_size + 1;
    // main grid x
    (1..max_x + 1)
        .into_par_iter()
        .flat_map(|grid_x| {
            // main grid y
            (1..max_x + 1).into_par_iter().map(move |grid_y| {
                // hashmap's struct tuple
                (
                    // coordinates tuple for the NxN square's top-left & square_size
                    (grid_x, grid_y, square_size),
                    // offsets for calculating sum of NxN
                    (0..square_size)
                        .flat_map(|offset_x| {
                            (0..square_size).map(move |offset_y| {
                                // get the value at this coord+offset
                                cells.get(&(grid_x + offset_x, grid_y + offset_y)).unwrap()
                            })
                        })
                        .sum::<i32>(),
                )
            })
        })
        .collect()
}
